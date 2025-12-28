macro_rules! LabelValue {
    () => {
        # [derive (Debug , Clone)] pub (crate) enum LabelValue { Const (String) , Ident (proc_macro2 :: TokenStream) , }
    };
}

LabelValue!();