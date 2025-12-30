// Generated macro for impl_21 (impl)
macro_rules! Depcrate_deimpl_21 {
() => {
// Module: crate::de
// Provides: {"impl_21"}
// Dependencies: {}
impl < E : de :: Error > Expected < E > for Header { # [inline] fn expected (self , kind : & 'static str) -> E { de :: Error :: invalid_type (match self { Header :: Positive (x) => de :: Unexpected :: Unsigned (x) , Header :: Negative (x) => de :: Unexpected :: Signed (x as i64 ^ ! 0) , Header :: Bytes (..) => de :: Unexpected :: Other ("bytes") , Header :: Text (..) => de :: Unexpected :: Other ("string") , Header :: Array (..) => de :: Unexpected :: Seq , Header :: Map (..) => de :: Unexpected :: Map , Header :: Tag (..) => de :: Unexpected :: Other ("tag") , Header :: Simple (simple :: FALSE) => de :: Unexpected :: Bool (false) , Header :: Simple (simple :: TRUE) => de :: Unexpected :: Bool (true) , Header :: Simple (simple :: NULL) => de :: Unexpected :: Other ("null") , Header :: Simple (simple :: UNDEFINED) => de :: Unexpected :: Other ("undefined") , Header :: Simple (..) => de :: Unexpected :: Other ("simple") , Header :: Float (x) => de :: Unexpected :: Float (x) , Header :: Break => de :: Unexpected :: Other ("break") , } , & kind ,) } }
};
}
