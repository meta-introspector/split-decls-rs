macro_rules! deps {
    () => {
        Diagnostic!();
        Applicability!();
        DiagnosticSpanMacroExpansion!();
        DiagnosticSpanLine!();
        Source!();
    };
}

macro_rules! DiagnosticSpan {
    () => {
        deps!();
        # [doc = " A section of the source code associated with a Diagnostic"] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct DiagnosticSpan { # [doc = " The file name or the macro name this diagnostic comes from."] pub file_name : String , # [doc = " The byte offset in the file where this diagnostic starts from."] pub byte_start : u32 , # [doc = " The byte offset in the file where this diagnostic ends."] pub byte_end : u32 , # [doc = " 1-based. The line in the file."] pub line_start : usize , # [doc = " 1-based. The line in the file."] pub line_end : usize , # [doc = " 1-based, character offset."] pub column_start : usize , # [doc = " 1-based, character offset."] pub column_end : usize , # [doc = " Is this a \"primary\" span -- meaning the point, or one of the points,"] # [doc = " where the error occurred?"] # [doc = ""] # [doc = " There are rare cases where multiple spans are marked as primary,"] # [doc = " e.g. \"immutable borrow occurs here\" and \"mutable borrow ends here\" can"] # [doc = " be two separate spans both \"primary\". Top (parent) messages should"] # [doc = " always have at least one primary span, unless it has 0 spans. Child"] # [doc = " messages may have 0 or more primary spans."] pub is_primary : bool , # [doc = " Source text from the start of line_start to the end of line_end."] pub text : Vec < DiagnosticSpanLine > , # [doc = " Label that should be placed at this location (if any)"] pub label : Option < String > , # [doc = " If we are suggesting a replacement, this will contain text"] # [doc = " that should be sliced in atop this span."] pub suggested_replacement : Option < String > , # [doc = " If the suggestion is approximate"] pub suggestion_applicability : Option < Applicability > , # [doc = " Macro invocations that created the code at this span, if any."] pub expansion : Option < Box < DiagnosticSpanMacroExpansion > > , }
    };
}

DiagnosticSpan!();