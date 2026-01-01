/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_USE_0001
/* FP:mod.rs-0002 */ use std :: io ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_USE_0002
/* FP:mod.rs-0004 */ use termcolor :: { Buffer , BufferWriter , ColorChoice } ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_MOD_0003
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_MOD_0004
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_STRUCT_0005
/* FP:mod.rs-0010 */ # [doc = " An AST representation of a Markdown document"] # [derive (Clone , Debug , Default , PartialEq)] pub struct MdStream < 'a > (Vec < MdTree < 'a > >) ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_IMPL_0006
/* FP:mod.rs-0012 */ impl < 'a > MdStream < 'a > { # [doc = " Parse a markdown string to a tokenstream"] # [must_use] pub fn parse_str (s : & str) -> MdStream < '_ > { parse :: entrypoint (s) } # [doc = " Write formatted output to a termcolor buffer"] pub fn write_termcolor_buf (& self , buf : & mut Buffer) -> io :: Result < () > { term :: entrypoint (self , buf) } }
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_FN_0007
/* FP:mod.rs-0014 */ # [doc = " Create a termcolor buffer with the `Always` color choice"] pub fn create_stdout_bufwtr () -> BufferWriter { BufferWriter :: stdout (ColorChoice :: Always) }
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_ENUM_0008
/* FP:mod.rs-0016 */ # [doc = " A single tokentree within a Markdown document"] # [derive (Clone , Debug , PartialEq)] pub enum MdTree < 'a > { # [doc = " Leaf types"] Comment (& 'a str) , CodeBlock { txt : & 'a str , lang : Option < & 'a str > , } , CodeInline (& 'a str) , Strong (& 'a str) , Emphasis (& 'a str) , Strikethrough (& 'a str) , PlainText (& 'a str) , # [doc = " [Foo](www.foo.com) or simple anchor <www.foo.com>"] Link { disp : & 'a str , link : & 'a str , } , # [doc = " `[Foo link][ref]`"] RefLink { disp : & 'a str , id : Option < & 'a str > , } , # [doc = " [ref]: www.foo.com"] LinkDef { id : & 'a str , link : & 'a str , } , # [doc = " Break bewtween two paragraphs (double `\\n`), not directly parsed but"] # [doc = " added later"] ParagraphBreak , # [doc = " Break bewtween two lines (single `\\n`)"] LineBreak , HorizontalRule , Heading (u8 , MdStream < 'a >) , OrderedListItem (u16 , MdStream < 'a >) , UnorderedListItem (MdStream < 'a >) , }
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_mod_IMPL_0009
/* FP:mod.rs-0018 */ impl < 'a > From < Vec < MdTree < 'a > > > for MdStream < 'a > { fn from (value : Vec < MdTree < 'a > >) -> Self { Self (value) } }