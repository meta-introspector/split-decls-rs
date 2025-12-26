#[derive(Default)]
pub struct MacroStat {
    /// Number of uses of the macro.
    pub uses: usize,

    /// Number of lines of code (when pretty-printed).
    pub lines: usize,

    /// Number of bytes of code (when pretty-printed).
    pub bytes: usize,
}
