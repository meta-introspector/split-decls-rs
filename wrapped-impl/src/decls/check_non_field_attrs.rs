macro_rules! deps {
    () => {
        Attrs!();
    };
}

macro_rules! check_non_field_attrs {
    () => {
        deps!();
        fn check_non_field_attrs (attrs : & Attrs) -> Result < () > { if let Some (from) = & attrs . from { return Err (Error :: new_spanned (from . original , "not expected here; the #[from] attribute belongs on a specific field" ,)) ; } if let Some (source) = & attrs . source { return Err (Error :: new_spanned (source . original , "not expected here; the #[source] attribute belongs on a specific field" ,)) ; } if let Some (backtrace) = & attrs . backtrace { return Err (Error :: new_spanned (backtrace , "not expected here; the #[backtrace] attribute belongs on a specific field" ,)) ; } if attrs . transparent . is_some () { if let Some (display) = & attrs . display { return Err (Error :: new_spanned (display . original , "cannot have both #[error(transparent)] and a display attribute" ,)) ; } if let Some (fmt) = & attrs . fmt { return Err (Error :: new_spanned (fmt . original , "cannot have both #[error(transparent)] and #[error(fmt = ...)]" ,)) ; } } else if let (Some (display) , Some (_)) = (& attrs . display , & attrs . fmt) { return Err (Error :: new_spanned (display . original , "cannot have both #[error(fmt = ...)] and a format arguments attribute" ,)) ; } Ok (()) }
    };
}

check_non_field_attrs!()