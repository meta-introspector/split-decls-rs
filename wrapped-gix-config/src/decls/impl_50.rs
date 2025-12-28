macro_rules! deps {
    () => {
        File!();
        Event!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl PartialEq for File < '_ > { fn eq (& self , other : & Self) -> bool { fn find_key < 'a > (mut it : impl Iterator < Item = & 'a Event < 'a > >) -> Option < & 'a section :: ValueName < 'a > > { it . find_map (| e | match e { Event :: SectionValueName (k) => Some (k) , _ => None , }) } fn collect_value < 'a > (it : impl Iterator < Item = & 'a Event < 'a > >) -> Cow < 'a , BStr > { let mut partial_value = BString :: default () ; let mut value = None ; for event in it { match event { Event :: SectionValueName (_) => break , Event :: Value (v) => { value = v . clone () . into () ; break ; } Event :: ValueNotDone (v) => partial_value . push_str (v . as_ref ()) , Event :: ValueDone (v) => { partial_value . push_str (v . as_ref ()) ; value = Some (partial_value . into ()) ; break ; } _ => () , } } value . map (normalize) . unwrap_or_default () } if self . section_order . len () != other . section_order . len () { return false ; } for (lhs , rhs) in self . section_order . iter () . zip (& other . section_order) . map (| (lhs , rhs) | (& self . sections [lhs] , & other . sections [rhs])) { if ! (lhs . header . name == rhs . header . name && lhs . header . subsection_name == rhs . header . subsection_name) { return false ; } let (mut lhs , mut rhs) = (lhs . body . 0 . iter () , rhs . body . 0 . iter ()) ; while let (Some (lhs_key) , Some (rhs_key)) = (find_key (& mut lhs) , find_key (& mut rhs)) { if lhs_key != rhs_key { return false ; } if collect_value (& mut lhs) != collect_value (& mut rhs) { return false ; } } } true } }
    };
}

impl_50!()