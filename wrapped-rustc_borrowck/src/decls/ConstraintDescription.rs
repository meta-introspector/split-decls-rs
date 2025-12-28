macro_rules! ConstraintDescription {
    () => {
        pub (crate) trait ConstraintDescription { fn description (& self) -> & 'static str ; }
    };
}

ConstraintDescription!();