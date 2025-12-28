macro_rules! deps {
    () => {
        OperationType!();
        Pos!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Parser error."] # [derive (Debug , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum Error { # [doc = " A syntax error occurred."] Syntax { # [doc = " The message of the error, nicely formatted with newlines."] message : String , # [doc = " The start position of the error."] start : Pos , # [doc = " The end position of the error, if present."] end : Option < Pos > , } , # [doc = " The schema contained multiple query, mutation or subscription roots."] MultipleRoots { # [doc = " The type of root that was duplicated."] root : OperationType , # [doc = " The position of the schema."] schema : Pos , # [doc = " The position of the second root."] pos : Pos , } , # [doc = " The schema contained no query root."] MissingQueryRoot { # [doc = " The position of the schema."] pos : Pos , } , # [doc = " Multiple operations were found in a document with an anonymous one."] MultipleOperations { # [doc = " The position of the anonymous operation."] anonymous : Pos , # [doc = " The position of the other operation."] operation : Pos , } , # [doc = " An operation is defined multiple times in a document."] OperationDuplicated { # [doc = " The name of the operation."] operation : Name , # [doc = " The position of the first definition."] first : Pos , # [doc = " The position of the second definition."] second : Pos , } , # [doc = " A fragment is defined multiple times in a document."] FragmentDuplicated { # [doc = " The name of the fragment."] fragment : Name , # [doc = " The position of the first definition."] first : Pos , # [doc = " The position of the second definition."] second : Pos , } , # [doc = " The document does not contain any operation."] MissingOperation , # [doc = " Recursion limit exceeded."] RecursionLimitExceeded , }
    };
}

Error!()