// Generated macro for simple_ast (function)
macro_rules! Depcrate_parser_tests_documentsimple_ast {
() => {
// Module: crate::parser::tests::document
// Provides: {"simple_ast"}
// Dependencies: {}
# [test] fn simple_ast () { assert_eq ! (parse_document ::< DefaultScalarValue > (r#"{
                node(id: 4) {
                    id
                    name
                }
            }"# ,) , vec ! [ast :: Definition :: Operation (Spanning :: start_end (& SourcePosition :: new (0 , 0 , 0) , & SourcePosition :: new (111 , 5 , 13) , ast :: Operation { operation_type : ast :: OperationType :: Query , name : None , description : None , variables_definition : None , directives : None , selection_set : vec ! [ast :: Selection :: Field (Spanning :: start_end (& SourcePosition :: new (18 , 1 , 16) , & SourcePosition :: new (97 , 4 , 17) , ast :: Field { alias : None , name : Spanning :: start_end (& SourcePosition :: new (18 , 1 , 16) , & SourcePosition :: new (22 , 1 , 20) , "node" ,) , arguments : Some (Spanning :: start_end (& SourcePosition :: new (22 , 1 , 20) , & SourcePosition :: new (29 , 1 , 27) , ast :: Arguments { items : vec ! [(Spanning :: start_end (& SourcePosition :: new (23 , 1 , 21) , & SourcePosition :: new (25 , 1 , 23) , "id" ,) , Spanning :: start_end (& SourcePosition :: new (27 , 1 , 25) , & SourcePosition :: new (28 , 1 , 26) , graphql :: input_value ! (4) ,) ,)] , } ,)) , directives : None , selection_set : Some (vec ! [ast :: Selection :: Field (Spanning :: start_end (& SourcePosition :: new (52 , 2 , 20) , & SourcePosition :: new (54 , 2 , 22) , ast :: Field { alias : None , name : Spanning :: start_end (& SourcePosition :: new (52 , 2 , 20) , & SourcePosition :: new (54 , 2 , 22) , "id" ,) , arguments : None , directives : None , selection_set : None , } ,)) , ast :: Selection :: Field (Spanning :: start_end (& SourcePosition :: new (75 , 3 , 20) , & SourcePosition :: new (79 , 3 , 24) , ast :: Field { alias : None , name : Spanning :: start_end (& SourcePosition :: new (75 , 3 , 20) , & SourcePosition :: new (79 , 3 , 24) , "name" ,) , arguments : None , directives : None , selection_set : None , } ,)) ,]) , } ,))] , } ,))]) }
};
}
