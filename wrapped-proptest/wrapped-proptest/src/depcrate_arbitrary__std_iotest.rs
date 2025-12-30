// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__std_iotest {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (buf_reader => BufReader < Repeat >, buf_writer => BufWriter < Sink >, line_writer => LineWriter < Sink >, chain => Chain < Empty , BufReader < Repeat >>, cursor => Cursor < Empty >, empty => Empty , sink => Sink , stderr => Stderr , stdin => Stdin , stdout => Stdout , lines => Lines < Empty >, repeat => Repeat , split => Split < Cursor < Vec < u8 >>>, take => Take < Repeat >, error_kind => ErrorKind , seek_from => SeekFrom , error => Error) ; }
};
}
