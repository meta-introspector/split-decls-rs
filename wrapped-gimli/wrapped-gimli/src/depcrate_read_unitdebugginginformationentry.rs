// Generated macro for DebuggingInformationEntry (struct)
macro_rules! Depcrate_read_unitDebuggingInformationEntry {
() => {
// Module: crate::read::unit
// Provides: {"DebuggingInformationEntry"}
// Dependencies: {}
# [doc = " A Debugging Information Entry (DIE)."] # [doc = ""] # [doc = " DIEs have a set of attributes and optionally have children DIEs as well."] # [derive (Clone , Debug)] pub struct DebuggingInformationEntry < 'abbrev , 'unit , R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { offset : UnitOffset < Offset > , attrs_slice : R , attrs_len : Cell < Option < Offset > > , abbrev : & 'abbrev Abbreviation , unit : & 'unit UnitHeader < R , Offset > , }
};
}
