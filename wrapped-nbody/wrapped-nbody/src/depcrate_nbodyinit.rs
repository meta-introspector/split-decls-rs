// Generated macro for init (function)
macro_rules! Depcrate_nbodyinit {
() => {
// Module: crate::nbody
// Provides: {"init"}
// Dependencies: {}
# [doc = " Simple function to create a lot of bodies."] # [doc = " Thank you, Larkins, for letting me use these umbers."] pub fn init (count : usize) -> BodyStates { let range : Vec < Number > = (0 .. count) . map (| i | i as Number) . collect () ; let ret = BodyStates { poss : range . iter () . map (| i | Position { x : 100. * (* i * 0.1) , y : 200. * (* i * 0.1) , z : 300. * (* i * 0.1) , }) . collect () , vels : range . iter () . map (| i | Velocity { dx : 400. + * i , dy : 500. + * i , dz : 600. + * i , }) . collect () , masses : range . iter () . map (| i | 10e6 * (* i + 100.2)) . collect () , } ; ret }
};
}
