mkuse!{use std :: io ;}
mkuse!{use termcolor :: { Buffer , BufferWriter , ColorChoice } ;}
mkmod!{parse, { 
                getname!(parse);
                getsrc!(parse);
                getpath!(parse);
                get_deps!(parse);
                get_crates!(parse);
                mkinclude!(parse);
                 
            }}
mkmod!{term, { 
                getname!(term);
                getsrc!(term);
                getpath!(term);
                get_deps!(term);
                get_crates!(term);
                mkinclude!(term);
                 
            }}
mkitem!{mkstruct!{# [doc = " An AST representation of a Markdown document"] # [derive (Clone , Debug , Default , PartialEq)] pub struct MdStream < 'a > (Vec < MdTree < 'a > >) ;}}
mkitem!{mkimpl!{impl < 'a > MdStream < 'a > { # [doc = " Parse a markdown string to a tokenstream"] # [must_use] pub fn parse_str (s : & str) -> MdStream < '_ > { parse :: entrypoint (s) } # [doc = " Write formatted output to a termcolor buffer"] pub fn write_termcolor_buf (& self , buf : & mut Buffer) -> io :: Result < () > { term :: entrypoint (self , buf) } }}}

macro_rules! create_stdout_bufwtr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_stdout_bufwtr in module {}", module_path!());
    };
}

mkfn!{
    create_stdout_bufwtr_introspect!();
    # [doc = " Create a termcolor buffer with the `Always` color choice"] pub fn create_stdout_bufwtr () -> BufferWriter { BufferWriter :: stdout (ColorChoice :: Always) }
}
mkitem!{mkenum!{# [doc = " A single tokentree within a Markdown document"] # [derive (Clone , Debug , PartialEq)] pub enum MdTree < 'a > { # [doc = " Leaf types"] Comment (& 'a str) , CodeBlock { txt : & 'a str , lang : Option < & 'a str > , } , CodeInline (& 'a str) , Strong (& 'a str) , Emphasis (& 'a str) , Strikethrough (& 'a str) , PlainText (& 'a str) , # [doc = " [Foo](www.foo.com) or simple anchor <www.foo.com>"] Link { disp : & 'a str , link : & 'a str , } , # [doc = " `[Foo link][ref]`"] RefLink { disp : & 'a str , id : Option < & 'a str > , } , # [doc = " [ref]: www.foo.com"] LinkDef { id : & 'a str , link : & 'a str , } , # [doc = " Break bewtween two paragraphs (double `\\n`), not directly parsed but"] # [doc = " added later"] ParagraphBreak , # [doc = " Break bewtween two lines (single `\\n`)"] LineBreak , HorizontalRule , Heading (u8 , MdStream < 'a >) , OrderedListItem (u16 , MdStream < 'a >) , UnorderedListItem (MdStream < 'a >) , }}}
mkitem!{mkimpl!{impl < 'a > From < Vec < MdTree < 'a > > > for MdStream < 'a > { fn from (value : Vec < MdTree < 'a > >) -> Self { Self (value) } }}}