// Generated macro for ScriptSource (struct)
macro_rules! Depcrate_frontmatterScriptSource {
() => {
// Module: crate::frontmatter
// Provides: {"ScriptSource"}
// Dependencies: {}
# [derive (Debug)] pub struct ScriptSource < 's > { # [doc = " The full file"] raw : & 's str , # [doc = " The `#!/usr/bin/env cargo` line, if present"] shebang : Option < Span > , # [doc = " The code fence opener (`---`)"] open : Option < Span > , # [doc = " Trailing text after `ScriptSource::open` that identifies the meaning of"] # [doc = " `ScriptSource::frontmatter`"] info : Option < Span > , # [doc = " The lines between `ScriptSource::open` and `ScriptSource::close`"] frontmatter : Option < Span > , # [doc = " The code fence closer (`---`)"] close : Option < Span > , # [doc = " All content after the frontmatter and shebang"] content : Span , }
};
}
