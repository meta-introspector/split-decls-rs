// Generated macro for RenderContext (struct)
macro_rules! Depcrate_renderRenderContext {
() => {
// Module: crate::render
// Provides: {"RenderContext"}
// Dependencies: {}
# [doc = " The context of a render call"] # [doc = ""] # [doc = " This context stores information of a render and a writer where generated"] # [doc = " content is written to."] # [doc = ""] # [derive (Clone)] pub struct RenderContext < 'reg : 'rc , 'rc > { dev_mode_templates : Option < & 'rc BTreeMap < String , Cow < 'rc , Template > > > , blocks : VecDeque < BlockContext < 'rc > > , modified_context : Option < Rc < Context > > , partials : BTreeMap < String , & 'rc Template > , partial_block_stack : VecDeque < & 'rc Template > , partial_block_depth : isize , local_helpers : BTreeMap < String , Rc < dyn HelperDef + Send + Sync + 'rc > > , # [doc = " current template name"] current_template : Option < & 'rc String > , # [doc = " root template name"] root_template : Option < & 'reg String > , disable_escape : bool , trailing_newline : bool , content_produced : bool , indent_before_write : bool , indent_string : Option < Cow < 'rc , str > > , }
};
}
