macro_rules! deps {
    () => {
        LookSet!();
        Properties!();
    };
}

macro_rules! PropertiesI {
    () => {
        deps!();
        # [doc = " The property definition. It is split out so that we can box it, and"] # [doc = " there by make `Properties` use less stack size. This is kind-of important"] # [doc = " because every HIR value has a `Properties` attached to it."] # [doc = ""] # [doc = " This does have the unfortunate consequence that creating any HIR value"] # [doc = " always leads to at least one alloc for properties, but this is generally"] # [doc = " true anyway (for pretty much all HirKinds except for look-arounds)."] # [derive (Clone , Debug , Eq , PartialEq)] struct PropertiesI { minimum_len : Option < usize > , maximum_len : Option < usize > , look_set : LookSet , look_set_prefix : LookSet , look_set_suffix : LookSet , look_set_prefix_any : LookSet , look_set_suffix_any : LookSet , utf8 : bool , explicit_captures_len : usize , static_explicit_captures_len : Option < usize > , literal : bool , alternation_literal : bool , }
    };
}

PropertiesI!()