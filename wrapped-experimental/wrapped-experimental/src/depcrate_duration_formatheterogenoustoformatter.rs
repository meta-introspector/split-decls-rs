// Generated macro for HeterogenousToFormatter (type)
macro_rules! Depcrate_duration_formatHeterogenousToFormatter {
() => {
// Module: crate::duration::format
// Provides: {"HeterogenousToFormatter"}
// Dependencies: {}
# [doc = " Exists to allow creating lists of heterogeneous [`Writeable`]s to pass to [`ListFormatter`]."] # [doc = " The (Unit, Decimal) pair is used to crerate [`FormattedUnit`]s."] type HeterogenousToFormatter = Either < DigitalDuration , (Unit , Decimal) > ;
};
}
