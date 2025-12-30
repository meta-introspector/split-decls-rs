include!(concat!(env!("CARGO_MANIFEST_DIR"), "/import_macros.rs"));

macro_rules! mkbin {
    () => {
        mod poly8x8x3_t {
            println!("Module poly8x8x3_t loaded");
            import_poly8x8x3_t!();
        }
        mod filter_map {
            println!("Module filter_map loaded");
            import_filter_map!();
        }
        mod either_is_async_read {
            println!("Module either_is_async_read loaded");
            import_either_is_async_read!();
        }
        mod archive_buf {
            println!("Module archive_buf loaded");
            import_archive_buf!();
        }
        mod is_type {
            println!("Module is_type loaded");
            import_is_type!();
        }
        mod to_node {
            println!("Module to_node loaded");
            import_to_node!();
        }
        mod AsciiExt {
            println!("Module AsciiExt loaded");
            import_AsciiExt!();
        }
        mod is_sized {
            println!("Module is_sized loaded");
            import_is_sized!();
        }
        mod proxy {
            println!("Module proxy loaded");
            import_proxy!();
        }
        mod leaf_decor {
            println!("Module leaf_decor loaded");
            import_leaf_decor!();
        }
        println!("mkbin executed with modules");
    };
}
