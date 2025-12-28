macro_rules! deps {
    () => {
        Field!();
        Directive!();
        Visitor!();
        KnownArgumentNames!();
        VisitorContext!();
        ArgsType!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for KnownArgumentNames < 'a > { fn enter_directive (& mut self , ctx : & mut VisitorContext < 'a > , directive : & 'a Positioned < Directive > ,) { self . current_args = ctx . registry . directives . get (directive . node . name . node . as_str ()) . map (| d | (& d . args , ArgsType :: Directive (& directive . node . name . node))) ; } fn exit_directive (& mut self , _ctx : & mut VisitorContext < 'a > , _directive : & 'a Positioned < Directive > ,) { self . current_args = None ; } fn enter_argument (& mut self , ctx : & mut VisitorContext < 'a > , name : & 'a Positioned < Name > , _value : & 'a Positioned < Value > ,) { if let Some ((args , arg_type)) = & self . current_args { if ! args . contains_key (name . node . as_str ()) { match arg_type { ArgsType :: Field { field_name , type_name , } => { ctx . report_error (vec ! [name . pos] , format ! ("Unknown argument \"{}\" on field \"{}\" of type \"{}\".{}" , name , field_name , type_name , if ctx . registry . enable_suggestions { self . get_suggestion (name . node . as_str ()) } else { String :: new () }) ,) ; } ArgsType :: Directive (directive_name) => { ctx . report_error (vec ! [name . pos] , format ! ("Unknown argument \"{}\" on directive \"{}\".{}" , name , directive_name , self . get_suggestion (name . node . as_str ())) ,) ; } } } } } fn enter_field (& mut self , ctx : & mut VisitorContext < 'a > , field : & 'a Positioned < Field >) { if let Some (parent_type) = ctx . parent_type () { if let Some (schema_field) = parent_type . field_by_name (& field . node . name . node) { self . current_args = Some ((& schema_field . args , ArgsType :: Field { field_name : & field . node . name . node , type_name : ctx . parent_type () . unwrap () . name () , } ,)) ; } } } fn exit_field (& mut self , _ctx : & mut VisitorContext < 'a > , _field : & 'a Positioned < Field >) { self . current_args = None ; } }
    };
}

impl_199!()