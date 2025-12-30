// Generated macro for default_decompression_commands (function)
macro_rules! Depcrate_decompressdefault_decompression_commands {
() => {
// Module: crate::decompress
// Provides: {"default_decompression_commands"}
// Dependencies: {}
fn default_decompression_commands () -> Vec < DecompressionCommand > { const ARGS_GZIP : & [& str] = & ["gzip" , "-d" , "-c"] ; const ARGS_BZIP : & [& str] = & ["bzip2" , "-d" , "-c"] ; const ARGS_XZ : & [& str] = & ["xz" , "-d" , "-c"] ; const ARGS_LZ4 : & [& str] = & ["lz4" , "-d" , "-c"] ; const ARGS_LZMA : & [& str] = & ["xz" , "--format=lzma" , "-d" , "-c"] ; const ARGS_BROTLI : & [& str] = & ["brotli" , "-d" , "-c"] ; const ARGS_ZSTD : & [& str] = & ["zstd" , "-q" , "-d" , "-c"] ; const ARGS_UNCOMPRESS : & [& str] = & ["uncompress" , "-c"] ; fn add (glob : & str , args : & [& str] , cmds : & mut Vec < DecompressionCommand >) { let bin = match resolve_binary (Path :: new (args [0])) { Ok (bin) => bin , Err (err) => { log :: debug ! ("{}" , err) ; return ; } } ; cmds . push (DecompressionCommand { glob : glob . to_string () , bin , args : args . iter () . skip (1) . map (| s | OsStr :: new (s) . to_os_string ()) . collect () , }) ; } let mut cmds = vec ! [] ; add ("*.gz" , ARGS_GZIP , & mut cmds) ; add ("*.tgz" , ARGS_GZIP , & mut cmds) ; add ("*.bz2" , ARGS_BZIP , & mut cmds) ; add ("*.tbz2" , ARGS_BZIP , & mut cmds) ; add ("*.xz" , ARGS_XZ , & mut cmds) ; add ("*.txz" , ARGS_XZ , & mut cmds) ; add ("*.lz4" , ARGS_LZ4 , & mut cmds) ; add ("*.lzma" , ARGS_LZMA , & mut cmds) ; add ("*.br" , ARGS_BROTLI , & mut cmds) ; add ("*.zst" , ARGS_ZSTD , & mut cmds) ; add ("*.zstd" , ARGS_ZSTD , & mut cmds) ; add ("*.Z" , ARGS_UNCOMPRESS , & mut cmds) ; cmds }
};
}
