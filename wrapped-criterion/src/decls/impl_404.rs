macro_rules! deps {
    () => {
        Criterion!();
        BenchmarkId!();
        Bencher!();
        Measurement!();
        BenchmarkGroup!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl < M > Criterion < M > where M : Measurement + 'static , { # [doc = " Benchmarks a function. For comparing multiple functions, see"] # [doc = " [`benchmark_group`](Self::benchmark_group)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use criterion::{criterion_group, criterion_main, Criterion};"] # [doc = ""] # [doc = " fn bench(c: &mut Criterion) {"] # [doc = "     // Setup (construct data, allocate memory, etc)"] # [doc = "     c.bench_function("] # [doc = "         \"function_name\","] # [doc = "         |b| b.iter(|| {"] # [doc = "             // Code to benchmark goes here"] # [doc = "         }),"] # [doc = "     );"] # [doc = " }"] # [doc = ""] # [doc = " criterion_group!(benches, bench);"] # [doc = " criterion_main!(benches);"] # [doc = " ```"] pub fn bench_function < F > (& mut self , id : & str , f : F) -> & mut Criterion < M > where F : FnMut (& mut Bencher < '_ , M >) , { self . benchmark_group (id) . bench_function (BenchmarkId :: no_function () , f) ; self } # [doc = " Benchmarks a function with an input. For comparing multiple functions or multiple inputs,"] # [doc = " see [`benchmark_group`](Self::benchmark_group)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};"] # [doc = ""] # [doc = " fn bench(c: &mut Criterion) {"] # [doc = "     // Setup (construct data, allocate memory, etc)"] # [doc = "     let input = 5u64;"] # [doc = "     c.bench_with_input("] # [doc = "         BenchmarkId::new(\"function_name\", input), &input,"] # [doc = "         |b, i| b.iter(|| {"] # [doc = "             // Code to benchmark using input `i` goes here"] # [doc = "         }),"] # [doc = "     );"] # [doc = " }"] # [doc = ""] # [doc = " criterion_group!(benches, bench);"] # [doc = " criterion_main!(benches);"] # [doc = " ```"] pub fn bench_with_input < F , I > (& mut self , id : BenchmarkId , input : & I , f : F) -> & mut Criterion < M > where F : FnMut (& mut Bencher < '_ , M > , & I) , { let group_name = id . function_name . expect ("Cannot use BenchmarkId::from_parameter with Criterion::bench_with_input. \
                 Consider using a BenchmarkGroup or BenchmarkId::new instead." ,) ; let parameter = id . parameter . unwrap () ; self . benchmark_group (group_name) . bench_with_input (BenchmarkId :: no_function_with_input (parameter) , input , f ,) ; self } }
    };
}

impl_404!();