macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! SharedEmitterMessage {
    () => {
        deps!();
        enum SharedEmitterMessage { Diagnostic (Diagnostic) , InlineAsmError (SpanData , String , Level , Option < (String , Vec < InnerSpan >) >) , Fatal (String) , }
    };
}

SharedEmitterMessage!()