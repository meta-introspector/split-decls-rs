// Generated macro for verify_objc_decl (function)
macro_rules! Depcrate_stmtverify_objc_decl {
() => {
// Module: crate::stmt
// Provides: {"verify_objc_decl"}
// Dependencies: {}
# [doc = " Takes one of:"] # [doc = " - `EntityKind::ObjCInterfaceDecl`"] # [doc = " - `EntityKind::ObjCProtocolDecl`"] # [doc = " - `EntityKind::ObjCCategoryDecl`"] fn verify_objc_decl (entity : & Entity < '_ > , _context : & Context < '_ >) { let parent_kind = entity . get_kind () ; immediate_children (entity , | entity , _span | { match (entity . get_kind () , parent_kind) { (EntityKind :: ObjCExplicitProtocolImpl , EntityKind :: ObjCProtocolDecl) => { } (EntityKind :: ObjCIvarDecl | EntityKind :: StructDecl | EntityKind :: UnionDecl , EntityKind :: ObjCInterfaceDecl ,) => { } (EntityKind :: ObjCSuperClassRef | EntityKind :: TypeRef , EntityKind :: ObjCInterfaceDecl ,) => { } (EntityKind :: ObjCSubclassingRestricted , EntityKind :: ObjCInterfaceDecl) => { } (EntityKind :: ObjCRootClass , EntityKind :: ObjCInterfaceDecl) => { debug ! ("parsing root class") ; } (EntityKind :: ObjCClassRef , EntityKind :: ObjCInterfaceDecl | EntityKind :: ObjCCategoryDecl ,) => { } (EntityKind :: TemplateTypeParameter , EntityKind :: ObjCInterfaceDecl | EntityKind :: ObjCCategoryDecl ,) => { } (EntityKind :: ObjCProtocolRef , _) => { } (EntityKind :: ObjCInstanceMethodDecl | EntityKind :: ObjCClassMethodDecl | EntityKind :: ObjCPropertyDecl , _ ,) => { } (EntityKind :: VisibilityAttr , _) => { } (EntityKind :: ObjCException , EntityKind :: ObjCInterfaceDecl) => { } (EntityKind :: UnexposedAttr , _) => { } (EntityKind :: AnnotateAttr , _) if entity . get_name () . unwrap () == "main-thread-only" => { } (_ , parent_kind) => error ! (? entity , ? parent_kind , "unknown in parent") , } }) ; }
};
}
