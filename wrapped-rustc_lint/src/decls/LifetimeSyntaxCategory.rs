macro_rules! LifetimeSyntaxCategory {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq)] enum LifetimeSyntaxCategory { Hidden , Elided , Named , }
    };
}

LifetimeSyntaxCategory!();