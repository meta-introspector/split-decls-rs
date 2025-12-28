macro_rules! deps {
    () => {
        ItemTree!();
        ItemVisibilities!();
        DefDatabase!();
    };
}

macro_rules! file_item_tree_query {
    () => {
        deps!();
        # [salsa_macros :: tracked (returns (deref))] pub (crate) fn file_item_tree_query (db : & dyn DefDatabase , file_id : HirFileId) -> Arc < ItemTree > { let _p = tracing :: info_span ! ("file_item_tree_query" , ? file_id) . entered () ; static EMPTY : OnceLock < Arc < ItemTree > > = OnceLock :: new () ; let ctx = lower :: Ctx :: new (db , file_id) ; let syntax = db . parse_or_expand (file_id) ; let mut item_tree = match_ast ! { match syntax { ast :: SourceFile (file) => { let top_attrs = RawAttrs :: new (db , & file , ctx . span_map ()) ; let mut item_tree = ctx . lower_module_items (& file) ; item_tree . top_attrs = top_attrs ; item_tree } , ast :: MacroItems (items) => { ctx . lower_module_items (& items) } , ast :: MacroStmts (stmts) => { ctx . lower_macro_stmts (stmts) } , _ => { if never ! (syntax . kind () == SyntaxKind :: ERROR , "{:?} from {:?} {}" , file_id , syntax , syntax) { return Default :: default () ; } panic ! ("cannot create item tree for file {file_id:?} from {syntax:?} {syntax}") ; } , } } ; let ItemTree { top_level , top_attrs , attrs , vis , big_data , small_data } = & item_tree ; if small_data . is_empty () && big_data . is_empty () && top_level . is_empty () && attrs . is_empty () && top_attrs . is_empty () && vis . arena . is_empty () { EMPTY . get_or_init (| | { Arc :: new (ItemTree { top_level : Box :: new ([]) , attrs : FxHashMap :: default () , small_data : FxHashMap :: default () , big_data : FxHashMap :: default () , top_attrs : RawAttrs :: EMPTY , vis : ItemVisibilities { arena : ThinVec :: new () } , }) }) . clone () } else { item_tree . shrink_to_fit () ; Arc :: new (item_tree) } }
    };
}

file_item_tree_query!()