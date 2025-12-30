// Generated macro for DesignatorWriter (struct)
macro_rules! Depcrate_fmt_friendly_printerDesignatorWriter {
() => {
// Module: crate::fmt::friendly::printer
// Provides: {"DesignatorWriter"}
// Dependencies: {}
# [doc = " An abstraction for writing the \"designator\" variant of the friendly format."] # [doc = ""] # [doc = " This takes care of computing some initial state and keeping track of some"] # [doc = " mutable state that influences printing. For example, whether to write a"] # [doc = " delimiter or not (one should only come after a unit that has been written)."] # [derive (Debug)] struct DesignatorWriter < 'p , 'w , W > { printer : & 'p SpanPrinter , wtr : & 'w mut W , desig : Designators , sign : Option < DirectionSign > , fmtint : DecimalFormatter , fmtfraction : FractionalFormatter , written_non_zero_unit : bool , }
};
}
