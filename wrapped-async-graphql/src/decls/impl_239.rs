macro_rules! deps {
    () => {
        Fields!();
        FindConflicts!();
        Field!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < 'a > FindConflicts < 'a , '_ > { pub fn find (& mut self , on_type : Option < & 'a str > , selection_set : & 'a Positioned < SelectionSet >) { for selection in & selection_set . node . items { match & selection . node { Selection :: Field (field) => { let output_name = field . node . alias . as_ref () . map (| name | & name . node) . unwrap_or_else (| | & field . node . name . node) ; self . add_output (on_type , & output_name , field) ; } Selection :: InlineFragment (inline_fragment) => { let on_type = inline_fragment . node . type_condition . as_ref () . map (| cond | cond . node . on . node . as_str ()) ; self . find (on_type , & inline_fragment . node . selection_set) ; } Selection :: FragmentSpread (fragment_spread) => { if let Some (fragment) = self . ctx . fragment (& fragment_spread . node . fragment_name . node) { let on_type = Some (fragment . node . type_condition . node . on . node . as_str ()) ; if ! self . visited . insert (fragment_spread . node . fragment_name . node . as_str ()) { continue ; } self . find (on_type , & fragment . node . selection_set) ; } } } } } fn add_output (& mut self , on_type : Option < & 'a str > , name : & 'a str , field : & 'a Positioned < Field > ,) { if let Some (prev_field) = self . outputs . get (& (on_type , name)) { if prev_field . node . name . node != field . node . name . node { self . ctx . report_error (vec ! [prev_field . pos , field . pos] , format ! ("Fields \"{}\" conflict because \"{}\" and \"{}\" are different fields. Use different aliases on the fields to fetch both if this was intentional." , name , prev_field . node . name . node , field . node . name . node)) ; } if prev_field . node . arguments . len () != field . node . arguments . len () { self . ctx . report_error (vec ! [prev_field . pos , field . pos] , format ! ("Fields \"{}\" conflict because they have differing arguments. Use different aliases on the fields to fetch both if this was intentional." , name)) ; } for (name , value) in & prev_field . node . arguments { match field . node . get_argument (& name . node) { Some (other_value) if value == other_value => { } _ => self . ctx . report_error (vec ! [prev_field . pos , field . pos] , format ! ("Fields \"{}\" conflict because they have differing arguments. Use different aliases on the fields to fetch both if this was intentional." , name)) , } } } else { self . outputs . insert ((on_type , name) , field) ; } } }
    };
}

impl_239!()