macro_rules! deps {
    () => {
        Safety!();
    };
}

macro_rules! HeaderSafety {
    () => {
        deps!();
        # [doc = " The actual safety specified in syntax. We may treat"] # [doc = " its safety different within the type system to create a"] # [doc = " \"sound by default\" system that needs checking this enum"] # [doc = " explicitly to allow unsafe operations."] # [derive (Copy , Clone , Debug , HashStable_Generic , PartialEq , Eq)] pub enum HeaderSafety { # [doc = " A safe function annotated with `#[target_features]`."] # [doc = " The type system treats this function as an unsafe function,"] # [doc = " but safety checking will check this enum to treat it as safe"] # [doc = " and allowing calling other safe target feature functions with"] # [doc = " the same features without requiring an additional unsafe block."] SafeTargetFeatures , Normal (Safety) , }
    };
}

HeaderSafety!()