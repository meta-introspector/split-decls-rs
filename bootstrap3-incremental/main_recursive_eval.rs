// Recursive evaluation for main
// Nix-like functional cache system

// Macro definitions
macro_rules! mkbin {
    (
        binary: $bin:expr,
        dependencies: [ $($dep:expr),* $(,)? ],
        total_deps: $total:expr,
        cache_entries: $cache:expr
    ) => {
        // Push each dependency onto the stack (each in its own module)
        $(
            mod stack_frame {
                use std::path::Path;
                include!($dep);
            }
            pub use stack_frame::*;
        )*
        
        pub fn eval() -> Result<(), Box<dyn std::error::Error>> {
            println!("🧬 EVALUATING STACK TOP");
            println!("📍 Binary: {}", $bin);
            println!("📈 Stack depth: {}", $total);
            println!("🔧 Cache entries: {}", $cache);
            
            // Eval the top of the stack (the main function)
            main()
        }
    };
}

macro_rules! mkdeclfn {
    (println!($($print_args:tt)*); fn $name:ident($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        pub fn $name($($args)*) -> $ret {
            println!($($print_args)*);
            $($body)*
        }
    };
    (fn $name:ident($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        pub fn $name($($args)*) -> $ret {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
    (fn $name:ident($($args:tt)*) { $($body:tt)* }) => {
        pub fn $name($($args)*) {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
    ($($body:tt)*) => {
        $($body)*
    };
}

// Dep: dep-ba0015e776660882 (hash: 8a3eed17dc596144)
mod dep_mod_0 {
    use serde :: { Deserialize , Serialize } ;
    use std :: collections :: HashMap ;
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rand/src/decls/std.rs"); // for std
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-impl/src/decls/Input.rs"); // for Input
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustc_macros/src/decls/Keyword.rs"); // for Keyword
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustc_span/src/decls/Ident.rs"); // for Ident
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustc_serialize/src/decls/Decodable.rs"); // for Decodable
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustc_span/src/decls/Symbol.rs"); // for Symbol
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustc_serialize/src/decls/Encodable.rs"); // for Encodable
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustc_span/src/decls/Span.rs"); // for Span
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-gix/src/decls/Clone.rs"); // for Clone
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-futures-util/src/decls/Empty.rs"); // for Empty
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustix/src/decls/Reader.rs"); // for Reader
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-zerocopy/src/decls/MaybeUninit.rs"); // for MaybeUninit
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustix/src/decls/RawDir.rs"); // for RawDir
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-tt/src/decls/Punct.rs"); // for Punct
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-tt/src/decls/Spacing.rs"); // for Spacing
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-async-graphql/src/decls/EOF.rs"); // for EOF
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-memchr/src/decls/Hash.rs"); // for Hash
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-plot/src/decls/Output.rs"); // for Output
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/iteration_times_figure.rs"); // for Path
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/POINT_SIZE.rs"); // for POINT_SIZE
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/MeasurementData.rs"); // for MeasurementData
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Data.rs"); // for Data
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Distributions.rs"); // for Distributions
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Estimates.rs"); // for Estimates
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/LabeledSample.rs"); // for LabeledSample
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/ComparisonData.rs"); // for ComparisonData
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Throughput.rs"); // for Throughput
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/DARK_BLUE.rs"); // for DARK_BLUE
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/DEFAULT_FONT.rs"); // for DEFAULT_FONT
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-serde_derive/src/decls/TRANSPARENT.rs"); // for TRANSPARENT
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/iteration_times_figure.rs"); // for Vec
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/SIZE.rs"); // for SIZE
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Sample.rs"); // for Sample
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-prettyplease/src/decls/data.rs"); // for data
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-split_rustc_data_structures/src/decls/Time.rs"); // for Time
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/ValueFormatter.rs"); // for ValueFormatter
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/impl_83.rs"); // for Result
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-env_logger/src/decls/Writer.rs"); // for Writer
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-env_logger/src/decls/BufferWriter.rs"); // for BufferWriter
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-env_logger/src/decls/WritableTarget.rs"); // for WritableTarget
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-env_logger/src/decls/WriteStyle.rs"); // for WriteStyle
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/BenchmarkId.rs"); // for BenchmarkId
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-git-wrapper-lib/src/decls/Deserialize.rs"); // for Deserialize
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-git-wrapper-lib/src/decls/Serialize.rs"); // for Serialize
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Comparison.rs"); // for String
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/ConfidenceInterval.rs"); // for ConfidenceInterval
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Plot.rs"); // for Plot
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Comparison.rs"); // for Comparison
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/CsvReportWriter.rs"); // for CsvReportWriter
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-futures-util/src/decls/Write.rs"); // for Write
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/FileCsvReport.rs"); // for FileCsvReport
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/Error.rs"); // for Error
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-tokio-test/src/decls/io.rs"); // for io
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-loom/src/decls/AccessError.rs"); // for AccessError
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-criterion/src/decls/error.rs"); // for error
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-allocator-api2/src/decls/Box.rs"); // for Box
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-zlib-rs/src/decls/Allocator.rs"); // for Allocator
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-zlib-rs/src/decls/c_api.rs"); // for c_api
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-tera/src/decls/Unique.rs"); // for Unique
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rust-crc32fast/src/decls/hash.rs"); // for hash
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rust-crc32fast/src/decls/Hasher.rs"); // for Hasher
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-lzma-rust2/src/decls/CRC32.rs"); // for CRC32
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-nom/src/decls/Check.rs"); // for Check
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-mockall_derive/src/decls/HashSet.rs"); // for HashSet
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-hashbrown/src/decls/hash_map.rs"); // for hash_map
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-hashlink/src/decls/DefaultHasher.rs"); // for DefaultHasher
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-futures-concurrency/src/decls/collections.rs"); // for collections
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-plot/src/decls/Default.rs"); // for Default
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-plot/src/decls/Properties.rs"); // for Properties
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-nom/src/decls/Not.rs"); // for Not
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-epoch/src/decls/Global.rs"); // for Global
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-epoch/src/decls/AtomicEpoch.rs"); // for AtomicEpoch
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-epoch/src/decls/Epoch.rs"); // for Epoch
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-tokio/src/decls/AtomicUsize.rs"); // for AtomicUsize
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-epoch/src/decls/Queue.rs"); // for Queue
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-utils/src/decls/CachePadded.rs"); // for CachePadded
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-split_rustc_data_structures/src/decls/Node.rs"); // for Node
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-epoch/src/decls/Atomic.rs"); // for Atomic
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-chrono/src/decls/Local.rs"); // for Local
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-sized-chunks/src/decls/arbitrary.rs"); // for arbitrary
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-chrono/src/decls/TimeZone.rs"); // for TimeZone
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-heapless/src/decls/defmt.rs"); // for defmt
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-split-decls-rs/src/decls/wrapped_arbitrary_decls_Arbitrary/trait/10/Arbitrary.rs"); // for Arbitrary
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-gix-archive/src/decls/Format.rs"); // for Format
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-gix-glob/src/decls/List.rs"); // for List
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-gix-glob/src/decls/Mapping.rs"); // for Mapping
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-aho-corasick/src/decls/Patterns.rs"); // for Patterns
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-toml/src/decls/Value.rs"); // for Value
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-bstr/src/decls/BString.rs"); // for BString
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-typenum/src/decls/Ord.rs"); // for Ord
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-rustc_public/src/decls/Pattern.rs"); // for Pattern
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-libm/src/decls/T.rs"); // for T
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-epoch/src/decls/SealedBag.rs"); // for SealedBag
    include!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs/output2/wrapped-crossbeam-epoch/src/decls/Bag.rs"); // for Bag

    mkdeclfn ! { println ! ("🔧 Calling function: main") ; fn main () -> Result < () , Box < dyn std :: error :: Error > > { let output3_dir = Path :: new ("../output3") ; let output4_dir = Path :: new ("../output4") ; println ! ("🔧 Bootstrap4: Proving ALL functions are wrapped!") ; println ! ("📂 Input:  {}" , output3_dir . display ()) ; println ! ("📂 Output: {}" , output4_dir . display ()) ; bootstrap_from_output3 (output3_dir , output4_dir) ? ; println ! ("✨ Bootstrap4 completed with ALL WRAPPED functions!") ; Ok (()) } }
}
pub use dep_mod_0::*;

mkbin! {
    binary: "main",
    dependencies: [
        "../output2/wrapped-split-decls-rs/src/decls/main/fn/4/main.rs", // dep-0: 8a3eed17dc596144
    ],
    total_deps: 1,
    cache_entries: 7
}
