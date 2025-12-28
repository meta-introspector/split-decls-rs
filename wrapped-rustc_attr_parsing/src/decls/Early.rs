macro_rules! deps {
    () => {
        Late!();
        ShouldEmit!();
    };
}

macro_rules! Early {
    () => {
        deps!();
        # [doc = " used when parsing attributes for miscellaneous things *before* ast lowering"] pub struct Early { # [doc = " Whether to emit errors or delay them as a bug"] # [doc = " For most attributes, the attribute will be parsed again in the `Late` stage and in this case the errors should be delayed"] # [doc = " But for some, such as `cfg`, the attribute will be removed before the `Late` stage so errors must be emitted"] pub emit_errors : ShouldEmit , }
    };
}

Early!();