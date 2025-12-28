# Complexity Scale Analysis for Split-Decls-RS

## Size Distribution Analysis

Based on the generated files in `output2/wrapped-unknown/src/decls`, we have sizes ranging from **86 bytes** to **12,225 bytes**.

## Proposed 1-10 Complexity Scale

| Scale | Size Range (bytes) | Description | Examples |
|-------|-------------------|-------------|----------|
| 1 | 86 - 200 | Trivial | Simple getters, basic constants |
| 2 | 201 - 400 | Simple | Basic functions, simple structs |
| 3 | 401 - 600 | Easy | Standard functions with logic |
| 4 | 601 - 800 | Moderate | Functions with some complexity |
| 5 | 801 - 1000 | Medium | Multi-step functions |
| 6 | 1001 - 1300 | Complex | Functions with branching logic |
| 7 | 1301 - 1700 | Advanced | Complex business logic |
| 8 | 1701 - 2500 | High | Large functions, multiple concerns |
| 9 | 2501 - 4000 | Very High | Complex algorithms, state machines |
| 10 | 4001+ | Extreme | Massive functions, generated code |

## Key Examples from Our Data

- **run_bootstrap_mode**: 1191 bytes = **Scale 6** (Complex)
- **run_bootstrap_mode** (variant): 2993 bytes = **Scale 9** (Very High)
- Smallest functions: 86-90 bytes = **Scale 1** (Trivial)
- Largest functions: 12,225 bytes = **Scale 10** (Extreme)

## Distribution Summary

- **Scale 1-2**: ~2,000 functions (simple utilities)
- **Scale 3-5**: ~8,000 functions (standard code)
- **Scale 6-8**: ~5,000 functions (complex logic)
- **Scale 9-10**: ~2,000 functions (very complex/generated)

This scale provides a good balance for organizing code by complexity rather than exact byte size.
