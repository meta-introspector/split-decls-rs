macro_rules! deps {
    () => {
        Features!();
    };
}

macro_rules! is_stable_diagnostic_attribute {
    () => {
        deps!();
        pub fn is_stable_diagnostic_attribute (sym : Symbol , _features : & Features) -> bool { match sym { sym :: on_unimplemented | sym :: do_not_recommend => true , _ => false , } }
    };
}

is_stable_diagnostic_attribute!()