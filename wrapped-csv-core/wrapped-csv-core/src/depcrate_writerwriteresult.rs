// Generated macro for WriteResult (enum)
macro_rules! Depcrate_writerWriteResult {
() => {
// Module: crate::writer
// Provides: {"WriteResult"}
// Dependencies: {}
# [doc = " The result of writing CSV data."] # [doc = ""] # [doc = " A value of this type is returned from every interaction with `Writer`. It"] # [doc = " informs the caller how to proceed, namely, by indicating whether more"] # [doc = " input should be given (`InputEmpty`) or if a bigger output buffer is needed"] # [doc = " (`OutputFull`)."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum WriteResult { # [doc = " This result occurs when all of the bytes from the given input have"] # [doc = " been processed."] InputEmpty , # [doc = " This result occurs when the output buffer was too small to process"] # [doc = " all of the input bytes. Generally, this means the caller must call"] # [doc = " the corresponding method again with the rest of the input and more"] # [doc = " room in the output buffer."] OutputFull , }
};
}
