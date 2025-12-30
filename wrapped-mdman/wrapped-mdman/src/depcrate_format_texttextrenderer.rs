// Generated macro for TextRenderer (struct)
macro_rules! Depcrate_format_textTextRenderer {
() => {
// Module: crate::format::text
// Provides: {"TextRenderer"}
// Dependencies: {}
struct TextRenderer < 'e > { output : String , indent : usize , # [doc = " The current line being written. Once a line break is encountered (such"] # [doc = " as starting a new paragraph), this will be written to `output` via"] # [doc = " `flush`."] line : String , # [doc = " The current word being written. Once a break is encountered (such as a"] # [doc = " space) this will be written to `line` via `flush_word`."] word : String , parser : EventIter < 'e > , # [doc = " The base URL used for relative URLs."] url : Option < Url > , table : Table , }
};
}
