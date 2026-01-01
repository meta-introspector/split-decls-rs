// SRC: ../rust/compiler/rustc_errors/src/markdown/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */
// A simple markdown parser that can write formatted text to the terminal
//
// Entrypoint is `MdStream::parse_str(...)`

use std::io;

use termcolor::{Buffer, BufferWriter, ColorChoice};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=MdStream | COMPLEXITY=5 | LINES=19 */

/// An AST representation of a Markdown document
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MdStream<'a>(Vec<MdTree<'a>>);

impl<'a> MdStream<'a> {
    /// Parse a markdown string to a tokenstream
    #[must_use]
    pub fn parse_str(s: &str) -> MdStream<'_> {
        parse::entrypoint(s)
    }

    /// Write formatted output to a termcolor buffer
    pub fn write_termcolor_buf(&self, buf: &mut Buffer) -> io::Result<()> {
        term::entrypoint(self, buf)
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=create_stdout_bufwtr | COMPLEXITY=2 | LINES=5 */

/// Create a termcolor buffer with the `Always` color choice
pub fn create_stdout_bufwtr() -> BufferWriter {
    BufferWriter::stdout(ColorChoice::Always)
}
/* AST_META: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=40 */

/// A single tokentree within a Markdown document
#[derive(Clone, Debug, PartialEq)]
pub enum MdTree<'a> {
    /// Leaf types
    Comment(&'a str),
    CodeBlock {
        txt: &'a str,
        lang: Option<&'a str>,
    },
    CodeInline(&'a str),
    Strong(&'a str),
    Emphasis(&'a str),
    Strikethrough(&'a str),
    PlainText(&'a str),
    /// [Foo](www.foo.com) or simple anchor <www.foo.com>
    Link {
        disp: &'a str,
        link: &'a str,
    },
    /// `[Foo link][ref]`
    RefLink {
        disp: &'a str,
        id: Option<&'a str>,
    },
    /// [ref]: www.foo.com
    LinkDef {
        id: &'a str,
        link: &'a str,
    },
    /// Break bewtween two paragraphs (double `\n`), not directly parsed but
    /// added later
    ParagraphBreak,
    /// Break bewtween two lines (single `\n`)
    LineBreak,
    HorizontalRule,
    Heading(u8, MdStream<'a>),
    OrderedListItem(u16, MdStream<'a>),
    UnorderedListItem(MdStream<'a>),
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6 */

impl<'a> From<Vec<MdTree<'a>>> for MdStream<'a> {
    fn from(value: Vec<MdTree<'a>>) -> Self {
        Self(value)
    }
}