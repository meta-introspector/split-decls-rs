macro_rules! deps {
    () => {
        Escape!();
        ContextValue!();
        StyledStr!();
    };
}

macro_rules! write_values_list {
    () => {
        deps!();
        # [cfg (feature = "error-context")] fn write_values_list (list_name : & 'static str , styled : & mut StyledStr , valid : & anstyle :: Style , possible_values : Option < & ContextValue > ,) { use std :: fmt :: Write as _ ; if let Some (ContextValue :: Strings (possible_values)) = possible_values { if ! possible_values . is_empty () { let _ = write ! (styled , "\n{TAB}[{list_name}: ") ; for (idx , val) in possible_values . iter () . enumerate () { if idx > 0 { styled . push_str (", ") ; } let _ = write ! (styled , "{valid}{}{valid:#}" , Escape (val)) ; } styled . push_str ("]") ; } } }
    };
}

write_values_list!()