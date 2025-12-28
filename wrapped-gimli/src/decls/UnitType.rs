macro_rules! deps {
    () => {
        DwoId!();
        DebugTypeSignature!();
        ReaderOffset!();
        UnitOffset!();
    };
}

macro_rules! UnitType {
    () => {
        deps!();
        # [doc = " This enum specifies the type of the unit and any type"] # [doc = " specific data carried in the header (e.g. the type"] # [doc = " signature/type offset of a type unit)."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum UnitType < Offset > where Offset : ReaderOffset , { # [doc = " In DWARF5, a unit with type `DW_UT_compile`. In previous DWARF versions,"] # [doc = " any unit appearing in the .debug_info section."] Compilation , # [doc = " In DWARF5, a unit with type `DW_UT_type`. In DWARF4, any unit appearing"] # [doc = " in the .debug_types section."] Type { # [doc = " The unique type signature for this type unit."] type_signature : DebugTypeSignature , # [doc = " The offset within this type unit where the type is defined."] type_offset : UnitOffset < Offset > , } , # [doc = " A unit with type `DW_UT_partial`. The root DIE of this unit should be a"] # [doc = " `DW_TAG_partial_unit`."] Partial , # [doc = " A unit with type `DW_UT_skeleton`. The enclosed dwo_id can be used to"] # [doc = " link this with the corresponding `SplitCompilation` unit in a dwo file."] # [doc = " NB: The non-standard GNU split DWARF extension to DWARF 4 will instead"] # [doc = " be a `Compilation` unit with the dwo_id present as an attribute on the"] # [doc = " root DIE."] Skeleton (DwoId) , # [doc = " A unit with type `DW_UT_split_compile`. The enclosed dwo_id can be used to"] # [doc = " link this with the corresponding `Skeleton` unit in the original binary."] # [doc = " NB: The non-standard GNU split DWARF extension to DWARF 4 will instead"] # [doc = " be a `Compilation` unit with the dwo_id present as an attribute on the"] # [doc = " root DIE."] SplitCompilation (DwoId) , # [doc = " A unit with type `DW_UT_split_type`. A split type unit is identical to a"] # [doc = " conventional type unit except for the section in which it appears."] SplitType { # [doc = " The unique type signature for this type unit."] type_signature : DebugTypeSignature , # [doc = " The offset within this type unit where the type is defined."] type_offset : UnitOffset < Offset > , } , }
    };
}

UnitType!()