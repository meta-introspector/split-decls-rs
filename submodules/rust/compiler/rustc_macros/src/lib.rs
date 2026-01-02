mkuse!{use proc_macro :: TokenStream ;}
mkuse!{use synstructure :: decl_derive ;}
mkmod!{current_version, { 
                getname!(current_version);
                getsrc!(current_version);
                getpath!(current_version);
                get_deps!(current_version);
                get_crates!(current_version);
                mkinclude!(current_version);
                 
            }}
mkmod!{diagnostics, { 
                getname!(diagnostics);
                getsrc!(diagnostics);
                getpath!(diagnostics);
                get_deps!(diagnostics);
                get_crates!(diagnostics);
                mkinclude!(diagnostics);
                 
            }}
mkmod!{extension, { 
                getname!(extension);
                getsrc!(extension);
                getpath!(extension);
                get_deps!(extension);
                get_crates!(extension);
                mkinclude!(extension);
                 
            }}
mkmod!{hash_stable, { 
                getname!(hash_stable);
                getsrc!(hash_stable);
                getpath!(hash_stable);
                get_deps!(hash_stable);
                get_crates!(hash_stable);
                mkinclude!(hash_stable);
                 
            }}
mkmod!{lift, { 
                getname!(lift);
                getsrc!(lift);
                getpath!(lift);
                get_deps!(lift);
                get_crates!(lift);
                mkinclude!(lift);
                 
            }}
mkmod!{print_attribute, { 
                getname!(print_attribute);
                getsrc!(print_attribute);
                getpath!(print_attribute);
                get_deps!(print_attribute);
                get_crates!(print_attribute);
                mkinclude!(print_attribute);
                 
            }}
mkmod!{query, { 
                getname!(query);
                getsrc!(query);
                getpath!(query);
                get_deps!(query);
                get_crates!(query);
                mkinclude!(query);
                 
            }}
mkmod!{serialize, { 
                getname!(serialize);
                getsrc!(serialize);
                getpath!(serialize);
                get_deps!(serialize);
                get_crates!(serialize);
                mkinclude!(serialize);
                 
            }}
mkmod!{symbols, { 
                getname!(symbols);
                getsrc!(symbols);
                getpath!(symbols);
                get_deps!(symbols);
                get_crates!(symbols);
                mkinclude!(symbols);
                 
            }}
mkmod!{try_from, { 
                getname!(try_from);
                getsrc!(try_from);
                getpath!(try_from);
                get_deps!(try_from);
                get_crates!(try_from);
                mkinclude!(try_from);
                 
            }}
mkmod!{type_foldable, { 
                getname!(type_foldable);
                getsrc!(type_foldable);
                getpath!(type_foldable);
                get_deps!(type_foldable);
                get_crates!(type_foldable);
                mkinclude!(type_foldable);
                 
            }}
mkmod!{type_visitable, { 
                getname!(type_visitable);
                getsrc!(type_visitable);
                getpath!(type_visitable);
                get_deps!(type_visitable);
                get_crates!(type_visitable);
                mkinclude!(type_visitable);
                 
            }}
mkmod!{visitable, { 
                getname!(visitable);
                getsrc!(visitable);
                getpath!(visitable);
                get_deps!(visitable);
                get_crates!(visitable);
                mkinclude!(visitable);
                 
            }}

macro_rules! current_rustc_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_rustc_version in module {}", module_path!());
    };
}

mkfn!{
    current_rustc_version_introspect!();
    # [proc_macro] pub fn current_rustc_version (input : TokenStream) -> TokenStream { current_version :: current_version (input) }
}

macro_rules! rustc_queries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_queries in module {}", module_path!());
    };
}

mkfn!{
    rustc_queries_introspect!();
    # [proc_macro] pub fn rustc_queries (input : TokenStream) -> TokenStream { query :: rustc_queries (input) }
}

macro_rules! symbols_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbols in module {}", module_path!());
    };
}

mkfn!{
    symbols_introspect!();
    # [proc_macro] pub fn symbols (input : TokenStream) -> TokenStream { symbols :: symbols (input . into ()) . into () }
}

macro_rules! extension_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extension in module {}", module_path!());
    };
}

mkfn!{
    extension_introspect!();
    # [doc = " Derive an extension trait for a given impl block. The trait name"] # [doc = " goes into the parenthesized args of the macro, for greppability."] # [doc = " For example:"] # [doc = " ```"] # [doc = " use rustc_macros::extension;"] # [doc = " #[extension(pub trait Foo)]"] # [doc = " impl i32 { fn hello() {} }"] # [doc = " ```"] # [doc = ""] # [doc = " expands to:"] # [doc = " ```"] # [doc = " pub trait Foo { fn hello(); }"] # [doc = " impl Foo for i32 { fn hello() {} }"] # [doc = " ```"] # [proc_macro_attribute] pub fn extension (attr : TokenStream , input : TokenStream) -> TokenStream { extension :: extension (attr , input) }
}
mkitem!{decl_derive ! ([HashStable , attributes (stable_hasher)] => hash_stable :: hash_stable_derive) ;}
mkitem!{decl_derive ! ([HashStable_Generic , attributes (stable_hasher)] => hash_stable :: hash_stable_generic_derive) ;}
mkitem!{decl_derive ! ([HashStable_NoContext] => # [doc = " `HashStable` implementation that has no `HashStableContext` bound and"] # [doc = " which adds `where` bounds for `HashStable` based off of fields and not"] # [doc = " generics. This is suitable for use in crates like `rustc_type_ir`."] hash_stable :: hash_stable_no_context_derive) ;}
mkitem!{decl_derive ! ([Decodable_NoContext] => serialize :: decodable_nocontext_derive) ;}
mkitem!{decl_derive ! ([Encodable_NoContext] => serialize :: encodable_nocontext_derive) ;}
mkitem!{decl_derive ! ([Decodable] => serialize :: decodable_derive) ;}
mkitem!{decl_derive ! ([Encodable] => serialize :: encodable_derive) ;}
mkitem!{decl_derive ! ([TyDecodable] => serialize :: type_decodable_derive) ;}
mkitem!{decl_derive ! ([TyEncodable] => serialize :: type_encodable_derive) ;}
mkitem!{decl_derive ! ([MetadataDecodable] => serialize :: meta_decodable_derive) ;}
mkitem!{decl_derive ! ([MetadataEncodable] => serialize :: meta_encodable_derive) ;}
mkitem!{decl_derive ! ([TypeFoldable , attributes (type_foldable)] => # [doc = " Derives `TypeFoldable` for the annotated `struct` or `enum` (`union` is not supported)."] # [doc = ""] # [doc = " The fold will produce a value of the same struct or enum variant as the input, with"] # [doc = " each field respectively folded using the `TypeFoldable` implementation for its type."] # [doc = " However, if a field of a struct or an enum variant is annotated with"] # [doc = " `#[type_foldable(identity)]` then that field will retain its incumbent value (and its"] # [doc = " type is not required to implement `TypeFoldable`)."] type_foldable :: type_foldable_derive) ;}
mkitem!{decl_derive ! ([TypeVisitable , attributes (type_visitable)] => # [doc = " Derives `TypeVisitable` for the annotated `struct` or `enum` (`union` is not supported)."] # [doc = ""] # [doc = " Each field of the struct or enum variant will be visited in definition order, using the"] # [doc = " `TypeVisitable` implementation for its type. However, if a field of a struct or an enum"] # [doc = " variant is annotated with `#[type_visitable(ignore)]` then that field will not be"] # [doc = " visited (and its type is not required to implement `TypeVisitable`)."] type_visitable :: type_visitable_derive) ;}
mkitem!{decl_derive ! ([Walkable , attributes (visitable)] => # [doc = " Derives `Walkable` for the annotated `struct` or `enum` (`union` is not supported)."] # [doc = ""] # [doc = " Each field of the struct or enum variant will be visited in definition order, using the"] # [doc = " `Walkable` implementation for its type. However, if a field of a struct or an enum"] # [doc = " variant is annotated with `#[visitable(ignore)]` then that field will not be"] # [doc = " visited (and its type is not required to implement `Walkable`)."] visitable :: visitable_derive) ;}
mkitem!{decl_derive ! ([Lift , attributes (lift)] => lift :: lift_derive) ;}
mkitem!{decl_derive ! ([Diagnostic , attributes (diag , help , help_once , note , note_once , warning , skip_arg , primary_span , label , subdiagnostic , suggestion , suggestion_short , suggestion_hidden , suggestion_verbose)] => diagnostics :: diagnostic_derive) ;}
mkitem!{decl_derive ! ([LintDiagnostic , attributes (diag , help , help_once , note , note_once , warning , skip_arg , primary_span , label , subdiagnostic , suggestion , suggestion_short , suggestion_hidden , suggestion_verbose)] => diagnostics :: lint_diagnostic_derive) ;}
mkitem!{decl_derive ! ([Subdiagnostic , attributes (label , help , help_once , note , note_once , warning , subdiagnostic , suggestion , suggestion_short , suggestion_hidden , suggestion_verbose , multipart_suggestion , multipart_suggestion_short , multipart_suggestion_hidden , multipart_suggestion_verbose , skip_arg , primary_span , suggestion_part , applicability)] => diagnostics :: subdiagnostic_derive) ;}
mkitem!{decl_derive ! { [TryFromU32] => # [doc = " Derives `TryFrom<u32>` for the annotated `enum`, which must have no fields."] # [doc = " Each variant maps to the value it would produce under an `as u32` cast."] # [doc = ""] # [doc = " The error type is `u32`."] try_from :: try_from_u32 }}
mkitem!{decl_derive ! { [PrintAttribute] => # [doc = " Derives `PrintAttribute` for `AttributeKind`."] # [doc = " This macro is pretty specific to `rustc_hir::attrs` and likely not that useful in"] # [doc = " other places. It's deriving something close to `Debug` without printing some extraneous"] # [doc = " things like spans."] print_attribute :: print_attribute }}