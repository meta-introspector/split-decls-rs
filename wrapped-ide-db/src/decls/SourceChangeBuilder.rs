macro_rules! deps {
    () => {
        TextEditBuilder!();
        SnippetBuilder!();
        AnnotationSnippet!();
        TreeMutator!();
        Command!();
        SourceChange!();
    };
}

macro_rules! SourceChangeBuilder {
    () => {
        deps!();
        pub struct SourceChangeBuilder { pub edit : TextEditBuilder , pub file_id : FileId , pub source_change : SourceChange , pub command : Option < Command > , # [doc = " Keeps track of all edits performed on each file"] pub file_editors : FxHashMap < FileId , SyntaxEditor > , # [doc = " Keeps track of which annotations correspond to which snippets"] pub snippet_annotations : Vec < (AnnotationSnippet , SyntaxAnnotation) > , # [doc = " Maps the original, immutable `SyntaxNode` to a `clone_for_update` twin."] pub mutated_tree : Option < TreeMutator > , # [doc = " Keeps track of where to place snippets"] pub snippet_builder : Option < SnippetBuilder > , }
    };
}

SourceChangeBuilder!()