// Generated macro for test_rustc_version (function)
macro_rules! Depcrate_teststest_rustc_version {
() => {
// Module: crate::tests
// Provides: {"test_rustc_version"}
// Dependencies: {}
# [test] fn test_rustc_version () { use self :: version :: Version ; let v = Version :: parse ("rustc 1.34.2 (6c2484dc3 2019-05-13)
binary: rustc
commit-hash: 6c2484dc3c532c052f159264e970278d8b77cdc9
commit-date: 2019-05-13
host: x86_64-apple-darwin
release: 1.34.2
LLVM version: 8.0" ,) . unwrap () ; assert_eq ! (v , Version :: stable (34 , 8)) ; let v = Version :: parse ("rustc 1.50.0 (cb75ad5db 2021-02-10)
binary: rustc
commit-hash: cb75ad5db02783e8b0222fee363c5f63f7e2cf5b
commit-date: 2021-02-10
host: aarch64-unknown-linux-gnu
release: 1.50.0" ,) . unwrap () ; assert_eq ! (v , Version :: stable (50 , 0)) ; let v = Version :: parse ("rustc 1.67.0 (fc594f156 2023-01-24)
binary: rustc
commit-hash: fc594f15669680fa70d255faec3ca3fb507c3405
commit-date: 2023-01-24
host: aarch64-apple-darwin
release: 1.67.0
LLVM version: 15.0.6" ,) . unwrap () ; assert_eq ! (v , Version :: stable (67 , 15)) ; let v = Version :: parse ("rustc 1.68.0-beta.2 (10b73bf73 2023-02-01)
binary: rustc
commit-hash: 10b73bf73a6b770cd92ad8ff538173bc3298411c
commit-date: 2023-02-01
host: aarch64-apple-darwin
release: 1.68.0-beta.2
LLVM version: 15.0.6" ,) . unwrap () ; assert_eq ! (v , Version :: stable (68 , 15)) ; let v = Version :: parse ("rustc 1.33.0-nightly (20c2cba61 2019-01-26)
binary: rustc
commit-hash: 20c2cba61dc83e612d25ed496025171caa3db30f
commit-date: 2019-01-26
host: x86_64-apple-darwin
release: 1.33.0-nightly
LLVM version: 8.0" ,) . unwrap () ; assert_eq ! (v . minor , 33) ; assert ! (v . nightly) ; assert_eq ! (v . llvm , 8) ; assert_eq ! (v . commit_date () . year , 2019) ; assert_eq ! (v . commit_date () . month , 1) ; assert_eq ! (v . commit_date () . day , 26) ; let v = Version :: parse ("rustc 1.69.0-nightly (bd39bbb4b 2023-02-07)
binary: rustc
commit-hash: bd39bbb4bb92df439bf6d85470e296cc6a47ffbd
commit-date: 2023-02-07
host: aarch64-apple-darwin
release: 1.69.0-nightly
LLVM version: 15.0.7" ,) . unwrap () ; assert_eq ! (v . minor , 69) ; assert ! (v . nightly) ; assert_eq ! (v . llvm , 15) ; assert_eq ! (v . commit_date () . year , 2023) ; assert_eq ! (v . commit_date () . month , 2) ; assert_eq ! (v . commit_date () . day , 7) ; let v = Version :: parse ("rustc 1.69.0-nightly (bd39bbb4b 2023-02-07)
binary: rustc
commit-hash: bd39bbb4bb92df439bf6d85470e296cc6a47ffbd
commit-date: 2023-02-07
host: aarch64-apple-darwin
release: 1.69.0-nightly
LLVM version: 15.0.7" ,) . unwrap () ; assert_eq ! (v . minor , 69) ; assert ! (v . nightly) ; assert_eq ! (v . llvm , 15) ; assert_eq ! (v . commit_date () . year , 2023) ; assert_eq ! (v . commit_date () . month , 2) ; assert_eq ! (v . commit_date () . day , 7) ; let v = Version :: parse ("rustc 1.69.0-dev
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-unknown-linux-gnu
release: 1.69.0-dev
LLVM version: 16.0.0" ,) . unwrap () ; assert_eq ! (v . minor , 69) ; assert ! (v . nightly) ; assert_eq ! (v . llvm , 16) ; assert_eq ! (v . commit_date () . year , 0) ; assert_eq ! (v . commit_date () . month , 0) ; assert_eq ! (v . commit_date () . day , 0) ; let v = Version :: parse ("rustc 1.48.0
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-unknown-linux-gnu
release: 1.48.0
LLVM version: 11.0" ,) . unwrap () ; assert_eq ! (v , Version :: stable (48 , 11)) ; let v = Version :: parse ("rustc 1.67.0 (fc594f156 2023-01-24) (Fedora 1.67.0-2.fc37)
binary: rustc
commit-hash: fc594f15669680fa70d255faec3ca3fb507c3405
commit-date: 2023-01-24
host: aarch64-unknown-linux-gnu
release: 1.67.0
LLVM version: 15.0.7" ,) . unwrap () ; assert_eq ! (v , Version :: stable (67 , 15)) ; let v = Version :: parse ("rustc 1.64.0
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-alpine-linux-musl
release: 1.64.0
LLVM version: 15.0.3" ,) . unwrap () ; assert_eq ! (v , Version :: stable (64 , 15)) ; }
};
}
