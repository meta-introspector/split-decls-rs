macro_rules! CompressionStrategy {
    () => {
        # [doc = " Strategy setting for compression."] # [doc = ""] # [doc = " The non-default settings offer some special-case compression variants."] # [repr (i32)] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum CompressionStrategy { # [doc = " Don't use any of the special strategies."] Default = 0 , # [doc = " Only use matches that are at least 5 bytes long."] Filtered = 1 , # [doc = " Don't look for matches, only huffman encode the literals."] HuffmanOnly = 2 , # [doc = " Only look for matches with a distance of 1, i.e do run-length encoding only."] RLE = 3 , # [doc = " Only use static/fixed blocks. (Blocks using the default huffman codes"] # [doc = " specified in the deflate specification.)"] Fixed = 4 , }
    };
}

CompressionStrategy!()