macro_rules! infer_edition {
    () => {
        fn infer_edition (file_path : & Path) -> Edition { let file_content = std :: fs :: read_to_string (file_path) . unwrap () ; if let Some (edition) = file_content . strip_prefix ("//@ edition: ") { edition [.. 4] . parse () . expect ("invalid edition directive") } else { Edition :: CURRENT } }
    };
}

infer_edition!();