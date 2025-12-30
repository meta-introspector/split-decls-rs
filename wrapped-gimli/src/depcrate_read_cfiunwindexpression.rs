// Generated macro for UnwindExpression (struct)
macro_rules! Depcrate_read_cfiUnwindExpression {
() => {
// Module: crate::read::cfi
// Provides: {"UnwindExpression"}
// Dependencies: {}
# [doc = " The location of a DWARF expression within an unwind section."] # [doc = ""] # [doc = " This is stored as an offset and length within the section instead of as a"] # [doc = " `Reader` to avoid lifetime issues when reusing [`UnwindContext`]."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use gimli::{EhFrame, EndianSlice, NativeEndian, Error, FrameDescriptionEntry, UnwindExpression, EvaluationResult};"] # [doc = " # fn foo() -> Result<(), Error> {"] # [doc = " # let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();"] # [doc = " # let fde: FrameDescriptionEntry<EndianSlice<NativeEndian>> = unimplemented!();"] # [doc = " # let unwind_expression: UnwindExpression<_> = unimplemented!();"] # [doc = " let expression = unwind_expression.get(&eh_frame)?;"] # [doc = " let mut evaluation = expression.evaluation(fde.cie().encoding());"] # [doc = " let mut result = evaluation.evaluate()?;"] # [doc = " loop {"] # [doc = "   match result {"] # [doc = "      EvaluationResult::Complete => break,"] # [doc = "      // Provide information to the evaluation."] # [doc = "      _ => { unimplemented!()}"] # [doc = "   }"] # [doc = " }"] # [doc = " let value = evaluation.value_result();"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct UnwindExpression < T : ReaderOffset > { # [doc = " The offset of the expression within the section."] pub offset : T , # [doc = " The length of the expression."] pub length : T , }
};
}
