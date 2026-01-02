mkuse!{pub use rmeta :: provide ;}
mkmod!{dependency_format, { 
                getname!(dependency_format);
                getsrc!(dependency_format);
                getpath!(dependency_format);
                get_deps!(dependency_format);
                get_crates!(dependency_format);
                mkinclude!(dependency_format);
                 
            }}
mkmod!{foreign_modules, { 
                getname!(foreign_modules);
                getsrc!(foreign_modules);
                getpath!(foreign_modules);
                get_deps!(foreign_modules);
                get_crates!(foreign_modules);
                mkinclude!(foreign_modules);
                 
            }}
mkmod!{native_libs, { 
                getname!(native_libs);
                getsrc!(native_libs);
                getpath!(native_libs);
                get_deps!(native_libs);
                get_crates!(native_libs);
                mkinclude!(native_libs);
                 
            }}
mkmod!{rmeta, { 
                getname!(rmeta);
                getsrc!(rmeta);
                getpath!(rmeta);
                get_deps!(rmeta);
                get_crates!(rmeta);
                mkinclude!(rmeta);
                 
            }}
mkmod!{creader, { 
                getname!(creader);
                getsrc!(creader);
                getpath!(creader);
                get_deps!(creader);
                get_crates!(creader);
                mkinclude!(creader);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{fs, { 
                getname!(fs);
                getsrc!(fs);
                getpath!(fs);
                get_deps!(fs);
                get_crates!(fs);
                mkinclude!(fs);
                 
            }}
mkmod!{locator, { 
                getname!(locator);
                getsrc!(locator);
                getpath!(locator);
                get_deps!(locator);
                get_crates!(locator);
                mkinclude!(locator);
                 
            }}
mkuse!{pub use creader :: { DylibError , load_symbol_from_dylib } ;}
mkuse!{pub use fs :: { METADATA_FILENAME , emit_wrapper_file } ;}
mkuse!{pub use native_libs :: { NativeLibSearchFallback , find_native_static_library , try_find_native_dynamic_library , try_find_native_static_library , walk_native_lib_search_dirs , } ;}
mkuse!{pub use rmeta :: { EncodedMetadata , METADATA_HEADER , encode_metadata , rendered_const } ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}