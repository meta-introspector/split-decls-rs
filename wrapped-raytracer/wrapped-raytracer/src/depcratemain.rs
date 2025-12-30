// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { const WIDTH : usize = 400 ; const HEIGHT : usize = 200 ; const NSAMPLES : usize = 100 ; let scene = create_scene () ; let lookfrom = Vec3 (20.0 * 0.47f32 . cos () , 20.0 * 0.47f32 . sin () , 3.0) ; let lookat = Vec3 (0.0 , 0.0 , 1.0) ; let vup = Vec3 (0.0 , 0.0 , 1.0) ; let focus_distance = (lookfrom - lookat) . length () ; let aperture = 0.3 ; let camera = Camera :: new (lookfrom , lookat , vup , 20.0 , WIDTH as f32 / HEIGHT as f32 , aperture , focus_distance ,) ; run_benchmark_group (| group | { group . register_benchmark ("raytracer" , | | { | | render :: render (scene . as_ref () , & camera , WIDTH , HEIGHT , NSAMPLES) }) ; }) ; }
};
}
