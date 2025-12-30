// Generated macro for emit_lint (function)
macro_rules! Depcrate_unused_io_amountemit_lint {
() => {
// Module: crate::unused_io_amount
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint (cx : & LateContext < '_ > , span : Span , at : HirId , op : IoOp , wild_cards : & [Span]) { let (msg , help) = match op { IoOp :: AsyncRead (false) => ("read amount is not handled" , Some ("use `AsyncReadExt::read_exact` instead, or handle partial reads") ,) , IoOp :: SyncRead (false) => ("read amount is not handled" , Some ("use `Read::read_exact` instead, or handle partial reads") ,) , IoOp :: SyncWrite (false) => ("written amount is not handled" , Some ("use `Write::write_all` instead, or handle partial writes") ,) , IoOp :: AsyncWrite (false) => ("written amount is not handled" , Some ("use `AsyncWriteExt::write_all` instead, or handle partial writes") ,) , IoOp :: SyncRead (true) | IoOp :: AsyncRead (true) => ("read amount is not handled" , None) , IoOp :: SyncWrite (true) | IoOp :: AsyncWrite (true) => ("written amount is not handled" , None) , } ; span_lint_hir_and_then (cx , UNUSED_IO_AMOUNT , at , span , msg , | diag | { if let Some (help_str) = help { diag . help (help_str) ; } for span in wild_cards { diag . span_note (* span , "the result is consumed here, but the amount of I/O bytes remains unhandled" ,) ; } }) ; }
};
}
