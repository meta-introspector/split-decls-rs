macro_rules! deps {
    () => {
        TypeCollector!();
        ExpressionInfo!();
        TypeUsageVisitor!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl TypeUsageVisitor { pub fn new (max_depth : usize) -> Self { TypeUsageVisitor { max_depth , current_depth : 0 , expressions : HashMap :: new () , struct_lattices : HashMap :: new () , enum_lattices : HashMap :: new () , impl_lattices : HashMap :: new () , } } pub fn get_expr_node_type (expr : & Expr) -> String { match expr { Expr :: Array (_) => "Array" . to_string () , Expr :: Assign (_) => "Assign" . to_string () , Expr :: Async (_) => "Async" . to_string () , Expr :: Await (_) => "Await" . to_string () , Expr :: Binary (_) => "Binary" . to_string () , Expr :: Block (_) => "Block" . to_string () , Expr :: Break (_) => "Break" . to_string () , Expr :: Call (_) => "Call" . to_string () , Expr :: Cast (_) => "Cast" . to_string () , Expr :: Closure (_) => "Closure" . to_string () , Expr :: Continue (_) => "Continue" . to_string () , Expr :: Field (_) => "Field" . to_string () , Expr :: ForLoop (_) => "ForLoop" . to_string () , Expr :: Group (_) => "Group" . to_string () , Expr :: If (_) => "If" . to_string () , Expr :: Index (_) => "Index" . to_string () , Expr :: Infer (_) => "Infer" . to_string () , Expr :: Let (_) => "Let" . to_string () , Expr :: Lit (_) => "Lit" . to_string () , Expr :: Loop (_) => "Loop" . to_string () , Expr :: Macro (_) => "Macro" . to_string () , Expr :: Match (_) => "Match" . to_string () , Expr :: MethodCall (_) => "MethodCall" . to_string () , Expr :: Paren (_) => "Paren" . to_string () , Expr :: Path (_) => "Path" . to_string () , Expr :: Range (_) => "Range" . to_string () , Expr :: Reference (_) => "Reference" . to_string () , Expr :: Repeat (_) => "Repeat" . to_string () , Expr :: Return (_) => "Return" . to_string () , Expr :: Struct (_) => "Struct" . to_string () , Expr :: Tuple (_) => "Tuple" . to_string () , Expr :: Unary (_) => "Unary" . to_string () , Expr :: Unsafe (_) => "Unsafe" . to_string () , Expr :: Verbatim (_) => "Verbatim" . to_string () , Expr :: While (_) => "While" . to_string () , Expr :: Yield (_) => "Yield" . to_string () , _ => "Unknown" . to_string () , } } pub fn extract_types_from_expr (& self , expr : & Expr) -> HashSet < String > { let mut types = HashSet :: new () ; let mut collector = TypeCollector { types : & mut types } ; collector . visit_expr (expr) ; types } pub fn process_expression (& mut self , expr : & Expr) { if self . current_depth <= self . max_depth { let expr_str = expr . to_token_stream () . to_string () ; let used_types = self . extract_types_from_expr (expr) ; let other_types_count = used_types . len () ; let node_type = Self :: get_expr_node_type (expr) ; let info = ExpressionInfo { expression_str : expr_str . clone () , depth : self . current_depth , used_types , other_types_count , node_type , } ; self . expressions . insert (expr_str , info) ; } } }
    };
}

impl_207!();