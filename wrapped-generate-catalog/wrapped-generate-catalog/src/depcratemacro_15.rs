// Generated macro for macro_15 (macro)
macro_rules! Depcratemacro_15 {
() => {
// Module: crate
// Provides: {"macro_15"}
// Dependencies: {}
lazy_static ! { static ref PARAMETERS_REGEX : Regex = r#"width=(\d+)\s+poly=0x([0-9a-fA-F]+)\s+init=0x([0-9a-fA-F]+)\s+refin=(false|true)\s+refout=(false|true)\s+xorout=0x([0-9a-fA-F]+)\s+check=0x([0-9a-fA-F]+)\s+residue=0x([0-9a-fA-F]+)\s+name="([^"]+)""# . parse () . unwrap () ; static ref NAME_REPLACE_REGEX : Regex = r"[-/]" . parse () . unwrap () ; }
};
}
