macro_rules! InFileWrapper {
    () => {
        # [doc = " `InFile<T>` stores a value of `T` inside a particular file/syntax tree."] # [doc = ""] # [doc = " Typical usages are:"] # [doc = ""] # [doc = " * `InFile<SyntaxNode>` -- syntax node in a file"] # [doc = " * `InFile<ast::FnDef>` -- ast node in a file"] # [doc = " * `InFile<TextSize>` -- offset in a file"] # [derive (Debug , PartialEq , Eq , Clone , Copy , Hash)] pub struct InFileWrapper < FileKind , T > { pub file_id : FileKind , pub value : T , }
    };
}

InFileWrapper!();