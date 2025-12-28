macro_rules! deps {
    () => {
        AstTraversalFunctor!();
        AstVisitor!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl PipelineFunctor < ParsedFile , AstStatistics , Config > for AstTraversalFunctor { fn map < 'writer > (& 'writer self , _writer : & 'writer mut (impl tokio :: io :: AsyncWriteExt + Unpin + Send) , input : ParsedFile , _config : & 'writer Option < Config > ,) -> Pin < Box < dyn Future < Output = Result < AstStatistics > > + Send + 'writer > > { Box :: pin (async move { let ParsedFile (parsed_code , file_path) = input ; let stats = tokio :: task :: spawn_blocking (move | | -> anyhow :: Result < AstStatistics > { let ast = syn :: parse_file (& parsed_code) . context ("Failed to parse code into AST for traversal") ? ; let mut visitor = AstVisitor :: new (file_path) ; syn :: visit :: visit_file (& mut visitor , & ast) ; Ok (visitor . stats) }) . await . context ("Blocking task for AST parsing failed") ? ? ; Ok (stats) }) } }
    };
}

impl_4!()