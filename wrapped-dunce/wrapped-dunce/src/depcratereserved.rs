// Generated macro for reserved (function)
macro_rules! Depcratereserved {
() => {
// Module: crate
// Provides: {"reserved"}
// Dependencies: {}
# [test] fn reserved () { assert ! (is_reserved ("CON")) ; assert ! (is_reserved ("con")) ; assert ! (is_reserved ("con.con")) ; assert ! (is_reserved ("COM4")) ; assert ! (is_reserved ("COM4.txt")) ; assert ! (is_reserved ("COM4 .txt")) ; assert ! (is_reserved ("con.")) ; assert ! (is_reserved ("con .")) ; assert ! (is_reserved ("con  ")) ; assert ! (is_reserved ("con . ")) ; assert ! (is_reserved ("con . .txt")) ; assert ! (is_reserved ("con.....txt")) ; assert ! (is_reserved ("PrN.....")) ; assert ! (is_reserved ("nul.tar.gz")) ; assert ! (! is_reserved (" PrN.....")) ; assert ! (! is_reserved (" CON")) ; assert ! (! is_reserved ("COM0")) ; assert ! (! is_reserved ("COM77")) ; assert ! (! is_reserved (" CON ")) ; assert ! (! is_reserved (".CON")) ; assert ! (! is_reserved ("@CON")) ; assert ! (! is_reserved ("not.CON")) ; assert ! (! is_reserved ("CON。")) ; }
};
}
