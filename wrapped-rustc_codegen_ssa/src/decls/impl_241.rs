macro_rules! deps {
    () => {
        SharedEmitterMain!();
        SharedEmitter!();
        SharedEmitterMessage!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl SharedEmitter { fn new () -> (SharedEmitter , SharedEmitterMain) { let (sender , receiver) = channel () ; (SharedEmitter { sender } , SharedEmitterMain { receiver }) } pub fn inline_asm_error (& self , span : SpanData , msg : String , level : Level , source : Option < (String , Vec < InnerSpan >) > ,) { drop (self . sender . send (SharedEmitterMessage :: InlineAsmError (span , msg , level , source))) ; } fn fatal (& self , msg : & str) { drop (self . sender . send (SharedEmitterMessage :: Fatal (msg . to_string ()))) ; } }
    };
}

impl_241!()