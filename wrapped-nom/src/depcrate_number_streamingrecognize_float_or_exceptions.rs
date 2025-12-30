// Generated macro for recognize_float_or_exceptions (function)
macro_rules! Depcrate_number_streamingrecognize_float_or_exceptions {
() => {
// Module: crate::number::streaming
// Provides: {"recognize_float_or_exceptions"}
// Dependencies: {}
# [doc (hidden)] pub fn recognize_float_or_exceptions < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Clone + Offset , T : Input + Compare < & 'static str > , < T as Input > :: Item : AsChar , { alt ((| i : T | { recognize_float :: < _ , E > (i . clone ()) . map_err (| e | match e { crate :: Err :: Error (_) => crate :: Err :: Error (E :: from_error_kind (i , ErrorKind :: Float)) , crate :: Err :: Failure (_) => crate :: Err :: Failure (E :: from_error_kind (i , ErrorKind :: Float)) , crate :: Err :: Incomplete (needed) => crate :: Err :: Incomplete (needed) , }) } , | i : T | { crate :: bytes :: streaming :: tag_no_case :: < _ , _ , E > ("nan") (i . clone ()) . map_err (| _ | crate :: Err :: Error (E :: from_error_kind (i , ErrorKind :: Float))) } , | i : T | { crate :: bytes :: streaming :: tag_no_case :: < _ , _ , E > ("infinity") (i . clone ()) . map_err (| _ | crate :: Err :: Error (E :: from_error_kind (i , ErrorKind :: Float))) } , | i : T | { crate :: bytes :: streaming :: tag_no_case :: < _ , _ , E > ("inf") (i . clone ()) . map_err (| _ | crate :: Err :: Error (E :: from_error_kind (i , ErrorKind :: Float))) } ,)) . parse (input) }
};
}
