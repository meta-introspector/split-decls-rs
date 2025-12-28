macro_rules! deps {
    () => {
        DeclInfo!();
    };
}

macro_rules! get_all_declarations {
    () => {
        deps!();
        pub fn get_all_declarations () -> Vec < DeclInfo > { DECL_REGISTRY . lock () . map (| r | r . declarations . clone ()) . unwrap_or_default () }
    };
}

get_all_declarations!()