macro_rules! deps {
    () => {
        UpdateCore!();
        Buffer!();
        InvalidOutputSize!();
        TruncSide!();
        BufferKindUser!();
    };
}

macro_rules! VariableOutputCore {
    () => {
        deps!();
        # [doc = " Core trait for hash functions with variable output size."] # [doc = ""] # [doc = " Maximum output size is equal to [`OutputSizeUser::OutputSize`]."] # [doc = " Users are expected to truncate result returned by the"] # [doc = " [`finalize_variable_core`] to `output_size` passed to the [`new`] method"] # [doc = " during construction. Truncation side is defined by the [`TRUNC_SIDE`]"] # [doc = " associated constant."] # [doc = ""] # [doc = " [`finalize_variable_core`]: VariableOutputCore::finalize_variable_core"] # [doc = " [`new`]: VariableOutputCore::new"] # [doc = " [`TRUNC_SIDE`]: VariableOutputCore::TRUNC_SIDE"] pub trait VariableOutputCore : UpdateCore + OutputSizeUser + BufferKindUser + Sized { # [doc = " Side which should be used in a truncated result."] const TRUNC_SIDE : TruncSide ; # [doc = " Initialize hasher state for given output size."] # [doc = ""] # [doc = " Returns [`InvalidOutputSize`] if `output_size` is not valid for"] # [doc = " the algorithm, e.g. if it's bigger than the [`OutputSize`]"] # [doc = " associated type."] # [doc = ""] # [doc = " [`OutputSize`]: OutputSizeUser::OutputSize"] fn new (output_size : usize) -> Result < Self , InvalidOutputSize > ; # [doc = " Finalize hasher and write full hashing result into the `out` buffer."] # [doc = ""] # [doc = " The result must be truncated to `output_size` used during hasher"] # [doc = " construction. Truncation side is defined by the [`TRUNC_SIDE`]"] # [doc = " associated constant."] # [doc = ""] # [doc = " [`TRUNC_SIDE`]: VariableOutputCore::TRUNC_SIDE"] fn finalize_variable_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) ; }
    };
}

VariableOutputCore!()