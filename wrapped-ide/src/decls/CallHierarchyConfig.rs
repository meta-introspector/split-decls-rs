macro_rules! CallHierarchyConfig {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct CallHierarchyConfig < 'a > { # [doc = " Whether to exclude tests from the call hierarchy"] pub exclude_tests : bool , pub minicore : MiniCore < 'a > , }
    };
}

CallHierarchyConfig!();