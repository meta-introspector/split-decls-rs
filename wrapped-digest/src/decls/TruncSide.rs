macro_rules! deps {
    () => {
        VariableOutputCore!();
    };
}

macro_rules! TruncSide {
    () => {
        deps!();
        # [doc = " Type which used for defining truncation side in the [`VariableOutputCore`]"] # [doc = " trait."] # [derive (Copy , Clone , Debug)] pub enum TruncSide { # [doc = " Truncate left side, i.e. `&out[..n]`."] Left , # [doc = " Truncate right side, i.e. `&out[m..]`."] Right , }
    };
}

TruncSide!()