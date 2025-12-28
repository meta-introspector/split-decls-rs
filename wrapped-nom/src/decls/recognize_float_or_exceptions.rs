macro_rules! deps {
    () => {
        Compare!();
        ErrorKind!();
        Parser!();
        Float!();
        ParseError!();
        Input!();
        Offset!();
        Error!();
        AsChar!();
        Err!();
    };
}

macro_rules! recognize_float_or_exceptions {
    () => {
        deps!();
        # [doc = " float number text parser that also recognizes \"nan\", \"infinity\" and \"inf\" (case insensitive)"] pub fn recognize_float_or_exceptions < T , E : ParseError < T > > () -> impl Parser < T , Output = T , Error = E > where T : Clone + Offset , T : Input + Compare < & 'static str > , < T as Input > :: Item : AsChar , { alt ((recognize_float :: < _ , E > () , | i : T | { crate :: bytes :: streaming :: tag_no_case :: < _ , _ , E > ("nan") (i . clone ()) . map_err (| _ | crate :: Err :: Error (E :: from_error_kind (i , ErrorKind :: Float))) } , | i : T | { crate :: bytes :: streaming :: tag_no_case :: < _ , _ , E > ("infinity") (i . clone ()) . map_err (| _ | crate :: Err :: Error (E :: from_error_kind (i , ErrorKind :: Float))) } , | i : T | { crate :: bytes :: streaming :: tag_no_case :: < _ , _ , E > ("inf") (i . clone ()) . map_err (| _ | crate :: Err :: Error (E :: from_error_kind (i , ErrorKind :: Float))) } ,)) }
    };
}

recognize_float_or_exceptions!()