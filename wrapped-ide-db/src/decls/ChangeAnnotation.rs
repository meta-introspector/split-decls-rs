macro_rules! ChangeAnnotation {
    () => {
        # [derive (Debug , Clone)] pub struct ChangeAnnotation { pub label : String , pub needs_confirmation : bool , pub description : Option < String > , }
    };
}

ChangeAnnotation!()