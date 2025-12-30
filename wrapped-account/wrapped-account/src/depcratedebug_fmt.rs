// Generated macro for debug_fmt (function)
macro_rules! Depcratedebug_fmt {
() => {
// Module: crate
// Provides: {"debug_fmt"}
// Dependencies: {}
fn debug_fmt < T : ReadableAccount > (item : & T , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("Account") ; f . field ("lamports" , & item . lamports ()) . field ("data.len" , & item . data () . len ()) . field ("owner" , & item . owner ()) . field ("executable" , & item . executable ()) . field ("rent_epoch" , & item . rent_epoch ()) ; debug_account_data (item . data () , & mut f) ; f . finish () }
};
}
