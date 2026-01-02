mkuse!{use crate :: ast :: ExprKind :: * ;}
mkuse!{use crate :: ast :: { self , MatchKind } ;}
mkuse!{use crate :: token :: Delimiter ;}

macro_rules! expr_is_complete_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_is_complete in module {}", module_path!());
    };
}

mkfn!{
    expr_is_complete_introspect!();
    # [doc = " This classification determines whether various syntactic positions break out"] # [doc = " of parsing the current expression (true) or continue parsing more of the"] # [doc = " same expression (false)."] # [doc = ""] # [doc = " For example, it's relevant in the parsing of match arms:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " match ... {"] # [doc = "     // Is this calling $e as a function, or is it the start of a new arm"] # [doc = "     // with a tuple pattern?"] # [doc = "     _ => $e ("] # [doc = "             ^                                                          )"] # [doc = ""] # [doc = "     // Is this an Index operation, or new arm with a slice pattern?"] # [doc = "     _ => $e ["] # [doc = "             ^                                                          ]"] # [doc = ""] # [doc = "     // Is this a binary operator, or leading vert in a new arm? Same for"] # [doc = "     // other punctuation which can either be a binary operator in"] # [doc = "     // expression or unary operator in pattern, such as `&` and `-`."] # [doc = "     _ => $e |"] # [doc = "             ^"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " If $e is something like `{}` or `if … {}`, then terminate the current"] # [doc = " arm and parse a new arm."] # [doc = ""] # [doc = " If $e is something like `path::to` or `(…)`, continue parsing the same"] # [doc = " arm."] # [doc = ""] # [doc = " *Almost* the same classification is used as an early bail-out for parsing"] # [doc = " statements. See `expr_requires_semi_to_be_stmt`."] pub fn expr_is_complete (e : & ast :: Expr) -> bool { matches ! (e . kind , If (..) | Match (..) | Block (..) | While (..) | Loop (..) | ForLoop { .. } | TryBlock (..) | ConstBlock (..)) }
}

macro_rules! expr_requires_semi_to_be_stmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_requires_semi_to_be_stmt in module {}", module_path!());
    };
}

mkfn!{
    expr_requires_semi_to_be_stmt_introspect!();
    # [doc = " Does this expression require a semicolon to be treated as a statement?"] # [doc = ""] # [doc = " The negation of this: \"can this expression be used as a statement without a"] # [doc = " semicolon\" -- is used as an early bail-out when parsing statements so that,"] # [doc = " for instance,"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " if true {...} else {...}"] # [doc = " |x| 5"] # [doc = " ```"] # [doc = ""] # [doc = " isn't parsed as `(if true {...} else {...} | x) | 5`."] # [doc = ""] # [doc = " Surprising special case: even though braced macro calls like `m! {}`"] # [doc = " normally do not introduce a boundary when found at the head of a match arm,"] # [doc = " they do terminate the parsing of a statement."] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " match ... {"] # [doc = "     _ => m! {} (),  // macro that expands to a function, which is then called"] # [doc = " }"] # [doc = ""] # [doc = " let _ = { m! {} () };  // macro call followed by unit"] # [doc = " ```"] pub fn expr_requires_semi_to_be_stmt (e : & ast :: Expr) -> bool { match & e . kind { MacCall (mac_call) => mac_call . args . delim != Delimiter :: Brace , _ => ! expr_is_complete (e) , } }
}

macro_rules! leading_labeled_expr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function leading_labeled_expr in module {}", module_path!());
    };
}

mkfn!{
    leading_labeled_expr_introspect!();
    # [doc = " Returns whether the leftmost token of the given expression is the label of a"] # [doc = " labeled loop or block, such as in `'inner: loop { break 'inner 1 } + 1`."] # [doc = ""] # [doc = " Such expressions are not allowed as the value of an unlabeled break."] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " 'outer: {"] # [doc = "     break 'inner: loop { break 'inner 1 } + 1;  // invalid syntax"] # [doc = ""] # [doc = "     break 'outer 'inner: loop { break 'inner 1 } + 1;  // okay"] # [doc = ""] # [doc = "     break ('inner: loop { break 'inner 1 } + 1);  // okay"] # [doc = ""] # [doc = "     break ('inner: loop { break 'inner 1 }) + 1;  // okay"] # [doc = " }"] # [doc = " ```"] pub fn leading_labeled_expr (mut expr : & ast :: Expr) -> bool { loop { match & expr . kind { Block (_ , label) | ForLoop { label , .. } | Loop (_ , label , _) | While (_ , _ , label) => { return label . is_some () ; } Assign (e , _ , _) | AssignOp (_ , e , _) | Await (e , _) | Use (e , _) | Binary (_ , e , _) | Call (e , _) | Cast (e , _) | Field (e , _) | Index (e , _ , _) | Match (e , _ , MatchKind :: Postfix) | Range (Some (e) , _ , _) | Try (e) => { expr = e ; } MethodCall (method_call) => { expr = & method_call . receiver ; } AddrOf (..) | Array (..) | Become (..) | Break (..) | Closure (..) | ConstBlock (..) | Continue (..) | FormatArgs (..) | Gen (..) | If (..) | IncludedBytes (..) | InlineAsm (..) | Let (..) | Lit (..) | MacCall (..) | Match (_ , _ , MatchKind :: Prefix) | OffsetOf (..) | Paren (..) | Path (..) | Range (None , _ , _) | Repeat (..) | Ret (..) | Struct (..) | TryBlock (..) | Tup (..) | Type (..) | Unary (..) | Underscore | Yeet (..) | Yield (..) | UnsafeBinderCast (..) | Err (..) | Dummy => return false , } } }
}
mkitem!{mkenum!{pub enum TrailingBrace < 'a > { # [doc = " Trailing brace in a macro call, like the one in `x as *const brace! {}`."] # [doc = " We will suggest changing the macro call to a different delimiter."] MacCall (& 'a ast :: MacCall) , # [doc = " Trailing brace in any other expression, such as `a + B {}`. We will"] # [doc = " suggest wrapping the innermost expression in parentheses: `a + (B {})`."] Expr (& 'a ast :: Expr) , }}}

macro_rules! expr_trailing_brace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_trailing_brace in module {}", module_path!());
    };
}

mkfn!{
    expr_trailing_brace_introspect!();
    # [doc = " If an expression ends with `}`, returns the innermost expression ending in the `}`"] pub fn expr_trailing_brace (mut expr : & ast :: Expr) -> Option < TrailingBrace < '_ > > { loop { match & expr . kind { AddrOf (_ , _ , e) | Assign (_ , e , _) | AssignOp (_ , _ , e) | Binary (_ , _ , e) | Break (_ , Some (e)) | Let (_ , e , _ , _) | Range (_ , Some (e) , _) | Ret (Some (e)) | Unary (_ , e) | Yeet (Some (e)) | Become (e) => { expr = e ; } Yield (kind) => match kind . expr () { Some (e) => expr = e , None => break None , } , Closure (closure) => { expr = & closure . body ; } Gen (..) | Block (..) | ForLoop { .. } | If (..) | Loop (..) | Match (..) | Struct (..) | TryBlock (..) | While (..) | ConstBlock (_) => break Some (TrailingBrace :: Expr (expr)) , Cast (_ , ty) => { break type_trailing_braced_mac_call (ty) . map (TrailingBrace :: MacCall) ; } MacCall (mac) => { break (mac . args . delim == Delimiter :: Brace) . then_some (TrailingBrace :: MacCall (mac)) ; } InlineAsm (_) | OffsetOf (_ , _) | IncludedBytes (_) | FormatArgs (_) => { break None ; } Break (_ , None) | Range (_ , None , _) | Ret (None) | Array (_) | Call (_ , _) | MethodCall (_) | Tup (_) | Lit (_) | Type (_ , _) | Await (_ , _) | Use (_ , _) | Field (_ , _) | Index (_ , _ , _) | Underscore | Path (_ , _) | Continue (_) | Repeat (_ , _) | Paren (_) | Try (_) | Yeet (None) | UnsafeBinderCast (..) | Err (_) | Dummy => { break None ; } } } }
}

macro_rules! type_trailing_braced_mac_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_trailing_braced_mac_call in module {}", module_path!());
    };
}

mkfn!{
    type_trailing_braced_mac_call_introspect!();
    # [doc = " If the type's last token is `}`, it must be due to a braced macro call, such"] # [doc = " as in `*const brace! { ... }`. Returns that trailing macro call."] fn type_trailing_braced_mac_call (mut ty : & ast :: Ty) -> Option < & ast :: MacCall > { loop { match & ty . kind { ast :: TyKind :: MacCall (mac) => { break (mac . args . delim == Delimiter :: Brace) . then_some (mac) ; } ast :: TyKind :: Ptr (mut_ty) | ast :: TyKind :: Ref (_ , mut_ty) | ast :: TyKind :: PinnedRef (_ , mut_ty) => { ty = & mut_ty . ty ; } ast :: TyKind :: UnsafeBinder (binder) => { ty = & binder . inner_ty ; } ast :: TyKind :: FnPtr (fn_ty) => match & fn_ty . decl . output { ast :: FnRetTy :: Default (_) => break None , ast :: FnRetTy :: Ty (ret) => ty = ret , } , ast :: TyKind :: Path (_ , path) => match path_return_type (path) { Some (trailing_ty) => ty = trailing_ty , None => break None , } , ast :: TyKind :: TraitObject (bounds , _) | ast :: TyKind :: ImplTrait (_ , bounds) => { match bounds . last () { Some (ast :: GenericBound :: Trait (bound)) => { match path_return_type (& bound . trait_ref . path) { Some (trailing_ty) => ty = trailing_ty , None => break None , } } Some (ast :: GenericBound :: Outlives (_) | ast :: GenericBound :: Use (..)) | None => { break None ; } } } ast :: TyKind :: Slice (..) | ast :: TyKind :: Array (..) | ast :: TyKind :: Never | ast :: TyKind :: Tup (..) | ast :: TyKind :: Paren (..) | ast :: TyKind :: Typeof (..) | ast :: TyKind :: Infer | ast :: TyKind :: ImplicitSelf | ast :: TyKind :: CVarArgs | ast :: TyKind :: Pat (..) | ast :: TyKind :: Dummy | ast :: TyKind :: Err (..) => break None , } } }
}

macro_rules! path_return_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_return_type in module {}", module_path!());
    };
}

mkfn!{
    path_return_type_introspect!();
    # [doc = " Returns the trailing return type in the given path, if it has one."] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " ::std::ops::FnOnce(&str) -> fn() -> *const c_void"] # [doc = "                             ^^^^^^^^^^^^^^^^^^^^^"] # [doc = " ```"] fn path_return_type (path : & ast :: Path) -> Option < & ast :: Ty > { let last_segment = path . segments . last () ? ; let args = last_segment . args . as_ref () ? ; match & * * args { ast :: GenericArgs :: Parenthesized (args) => match & args . output { ast :: FnRetTy :: Default (_) => None , ast :: FnRetTy :: Ty (ret) => Some (ret) , } , ast :: GenericArgs :: AngleBracketed (_) | ast :: GenericArgs :: ParenthesizedElided (_) => None , } }
}