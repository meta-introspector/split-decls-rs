macro_rules! deps {
    () => {
        VariableOutputCore!();
    };
}

macro_rules! VariableOutputCoreCustomized {
    () => {
        deps!();
        # [doc = " Trait adding customization string to hash functions with variable output."] pub trait VariableOutputCoreCustomized : VariableOutputCore { # [doc = " Create new hasher instance with the given customization string and output size."] fn new_customized (customization : & [u8] , output_size : usize) -> Self ; }
    };
}

VariableOutputCoreCustomized!()